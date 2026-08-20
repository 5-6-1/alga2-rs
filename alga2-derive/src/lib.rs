//! `#[derive(Alga)]` — one line brings a struct into the alga2 tower.
//!
//! Generates the algebraic chain as component-wise forwarders. The target
//! level is controlled by `#[alga(level = "...")]` (default `"Ring"`; the
//! field types must reach it — Rust rejects a level the fields cannot
//! satisfy, which is exactly the check you want):
//!
//! - `"Monoid"` — additive Magma → Monoid, multiplicative Magma → Monoid
//! - `"Group"` — + additive Group/AbelianGroup
//! - `"Ring"` (default) — + Semiring → CommutativeRing, plus Module/VectorSpace
//! - `"Field"` — + DivisionRing → Field (field-typed fields only)
//!
//! ```
//! use alga2::op::Additive;
//! use alga2::tower::Magma;
//! use alga2_derive::Alga;
//!
//! #[derive(Alga)]
//! #[alga(level = "Field")]
//! struct Vec2 { x: f64, y: f64 }
//!
//! let a = Vec2 { x: 1.0, y: 2.0 };
//! let b = Vec2 { x: 3.0, y: 4.0 };
//! assert_eq!(<Vec2 as Magma<Additive>>::combine(&a, &b).x, 4.0);
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Index, Type};

/// Generates the tower impls for a struct.
#[proc_macro_derive(Alga, attributes(alga))]
pub fn derive_alga(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(ts) => ts.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// The target level parsed from `#[alga(level = "...")]`.
#[derive(PartialEq, Clone, Copy)]
enum Level {
    Monoid,
    Group,
    Ring,
    Field,
}

impl Level {
    fn parse(input: &DeriveInput) -> syn::Result<Self> {
        let mut level = Level::Ring;
        for attr in &input.attrs {
            if attr.path().is_ident("alga") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("level") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        level = match s.value().as_str() {
                            "Monoid" => Level::Monoid,
                            "Group" => Level::Group,
                            "Ring" => Level::Ring,
                            "Field" => Level::Field,
                            other => {
                                return Err(meta.error(format!(
                                    "unknown alga level `{other}` (expected Monoid, Group, Ring or Field)"
                                )))
                            }
                        };
                        Ok(())
                    } else {
                        Err(meta.error("unknown `#[alga(...)]` option (only `level` is supported)"))
                    }
                })?;
            }
        }
        Ok(level)
    }
}

/// A struct field: its type, and how to name it in `self.x` / `self.0`.
struct FieldInfo {
    ty: Type,
    suffix: TS,
    name: Option<Ident>,
}

fn expand(input: &DeriveInput) -> syn::Result<TS> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_g, ty_g, where_g) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(s) => fields_of(&s.fields)?,
        Data::Enum(_) | Data::Union(_) => {
            return Err(syn::Error::new_spanned(input, "#[derive(Alga)] supports structs only"))
        }
    };

    let level = Level::parse(input)?;
    let g = Gen { name, impl_g: &impl_g, ty_g: &ty_g, where_g };
    let mut out = TS::new();
    // The additive ladder runs to `max_level`; the multiplicative ladder
    // stops at Monoid (numerics have no multiplicative inverses in general —
    // the ring/field levels are the two-operator chain below).
    let add_max = if level == Level::Monoid { "Monoid" } else { "Group" };
    out.extend(g.single_op_chain(&fields, "Additive", add_max));
    out.extend(g.single_op_chain(&fields, "Multiplicative", "Monoid"));
    if matches!(level, Level::Ring | Level::Field) {
        out.extend(g.two_op_chain(&fields, level));
        out.extend(g.module_level(&fields));
    }
    Ok(out)
}

fn fields_of(fields: &Fields) -> syn::Result<Vec<FieldInfo>> {
    let mut out = vec![];
    match fields {
        Fields::Named(n) => {
            for f in &n.named {
                let id = f.ident.clone().unwrap();
                out.push(FieldInfo { ty: f.ty.clone(), suffix: id.to_token_stream(), name: Some(id) });
            }
        }
        Fields::Unnamed(u) => {
            for (i, f) in u.unnamed.iter().enumerate() {
                let idx = Index::from(i);
                out.push(FieldInfo { ty: f.ty.clone(), suffix: idx.to_token_stream(), name: None });
            }
        }
        Fields::Unit => {
            return Err(syn::Error::new_spanned(fields, "#[derive(Alga)] needs at least one field"))
        }
    }
    if out.is_empty() {
        return Err(syn::Error::new_spanned(fields, "#[derive(Alga)] needs at least one field"));
    }
    Ok(out)
}

/// `self.<suffix>` — the receiver-side field access.
fn self_access(f: &FieldInfo) -> TS {
    let s = &f.suffix;
    quote!(self.#s)
}

/// `rhs.<suffix>` — the other-side field access.
fn rhs_access(f: &FieldInfo) -> TS {
    let s = &f.suffix;
    quote!(rhs.#s)
}

/// `Self { x: <e0>, y: <e1> }` for named fields, `Self(<e0>, <e1>)` for tuples.
fn construct(fields: &[FieldInfo], exprs: &[TS]) -> TS {
    match fields.first().map(|f| f.name.is_some()) {
        Some(true) => {
            let pairs = fields.iter().zip(exprs).map(|(f, e)| {
                let n = f.name.as_ref().unwrap();
                quote!(#n: #e)
            });
            quote!(Self { #(#pairs),* })
        }
        _ => quote!(Self( #(#exprs),* )),
    }
}

/// Deduplicated field types (a `where` bound per distinct type).
fn field_types(fields: &[FieldInfo]) -> Vec<&Type> {
    let mut seen = std::collections::HashSet::new();
    fields.iter().map(|f| &f.ty).filter(|t| seen.insert(t.to_token_stream().to_string())).collect()
}

fn op_path(op: &str) -> TS {
    let op = Ident::new(op, proc_macro2::Span::call_site());
    quote!(::alga2::op::#op)
}

fn trait_path(name: &str) -> TS {
    let name = Ident::new(name, proc_macro2::Span::call_site());
    quote!(::alga2::tower::#name)
}

struct Gen<'a> {
    name: &'a Ident,
    impl_g: &'a syn::ImplGenerics<'a>,
    ty_g: &'a syn::TypeGenerics<'a>,
    where_g: Option<&'a syn::WhereClause>,
}

impl Gen<'_> {
    /// Per-field `T: Trait<Op>` bound list.
    fn bounds(&self, fields: &[FieldInfo], trait_name: &str, op: &str) -> Vec<TS> {
        let tr = trait_path(trait_name);
        let op = op_path(op);
        field_types(fields).iter().map(|ty| quote!(#ty: #tr<#op>)).collect()
    }

    /// Magma → `max_level` for one operator, component-wise.
    fn single_op_chain(&self, fields: &[FieldInfo], op: &str, max_level: &str) -> TS {
        let n = self.name;
        let impl_g = self.impl_g;
        let ty_g = self.ty_g;
        let where_g = self.where_g;
        let op_ts = op_path(op);
        let mag = trait_path("Magma");
        let semi = trait_path("Semigroup");
        let quasi = trait_path("Quasigroup");
        let mon = trait_path("Monoid");
        let loop_t = trait_path("Loop");
        let grp = trait_path("Group");
        let abel = trait_path("AbelianGroup");

        let combine_exprs = fields.iter().map(|f| {
            let a = self_access(f);
            let b = rhs_access(f);
            let ty = &f.ty;
            quote!(<#ty as #mag<#op_ts>>::combine(&#a, &#b))
        }).collect::<Vec<_>>();
        let identity_exprs = fields.iter().map(|f| {
            let ty = &f.ty;
            quote!(<#ty as #mon<#op_ts>>::identity())
        }).collect::<Vec<_>>();
        let inverse_exprs = fields.iter().map(|f| {
            let a = self_access(f);
            quote!(#a.inverse())
        }).collect::<Vec<_>>();

        let magma_b = self.bounds(fields, "Magma", op);
        let monoid_b = self.bounds(fields, "Monoid", op);
        let group_b = self.bounds(fields, "Group", op);

        let c_combine = construct(fields, &combine_exprs);
        let c_identity = construct(fields, &identity_exprs);
        let c_inverse = construct(fields, &inverse_exprs);

        // The quasigroup ladder (Quasigroup → Loop) is additive-only in the
        // crate too; the multiplicative side stops at Monoid.
        let quasigroup_ladder = (op == "Additive").then(|| quote! {
            impl #impl_g #quasi<#op_ts> for #n #ty_g
            where #where_g #(#magma_b),*
            {}
            impl #impl_g #loop_t<#op_ts> for #n #ty_g
            where #where_g #(#monoid_b),*
            {}
        });

        // Group/AbelianGroup only when the ladder runs that far.
        let group_ladder = (max_level == "Group").then(|| quote! {
            impl #impl_g #grp<#op_ts> for #n #ty_g
            where #where_g #(#group_b),*
            {
                fn inverse(&self) -> Self { #c_inverse }
            }
            impl #impl_g #abel<#op_ts> for #n #ty_g
            where #where_g #(#group_b),*
            {}
        });

        quote! {
            impl #impl_g #mag<#op_ts> for #n #ty_g
            where #where_g #(#magma_b),*
            {
                fn combine(&self, rhs: &Self) -> Self { #c_combine }
            }
            impl #impl_g #semi<#op_ts> for #n #ty_g
            where #where_g #(#magma_b),*
            {}
            #quasigroup_ladder
            impl #impl_g #mon<#op_ts> for #n #ty_g
            where #where_g #(#monoid_b),*
            {
                fn identity() -> Self { #c_identity }
            }
            #group_ladder
        }
    }

    /// Semiring → Field (or through CommutativeRing for `Ring` level),
    /// component-wise.
    fn two_op_chain(&self, fields: &[FieldInfo], level: Level) -> TS {
        let n = self.name;
        let impl_g = self.impl_g;
        let ty_g = self.ty_g;
        let where_g = self.where_g;

        let semiring_b = field_types(fields).iter().map(|ty| {
            quote!(#ty: ::alga2::tower::Semiring<::alga2::op::Additive, ::alga2::op::Multiplicative>)
        }).collect::<Vec<_>>();
        let field_b = field_types(fields).iter().map(|ty| {
            quote!(#ty: ::alga2::tower::Field<::alga2::op::Additive, ::alga2::op::Multiplicative>)
        }).collect::<Vec<_>>();

        // CommutativeRing needs the fields to be commutative rings; at `Ring`
        // level we use the semiring bound (a commutative ring implies it).
        let comm_b = if level == Level::Field {
            &field_b
        } else {
            &semiring_b
        };

        let inv_exprs = fields.iter().map(|f| {
            let a = self_access(f);
            quote!(#a.inv())
        }).collect::<Vec<_>>();
        let c_inv = construct(fields, &inv_exprs);

        // DivisionRing/Field only at `Field` level.
        let field_ladder = (level == Level::Field).then(|| quote! {
            impl #impl_g ::alga2::tower::DivisionRing<::alga2::op::Additive, ::alga2::op::Multiplicative> for #n #ty_g
            where #where_g #(#field_b),*
            {
                fn inv(&self) -> Self { #c_inv }
            }
            impl #impl_g ::alga2::tower::Field<::alga2::op::Additive, ::alga2::op::Multiplicative> for #n #ty_g
            where #where_g #(#field_b),*
            {}
        });

        quote! {
            impl #impl_g ::alga2::tower::Semiring<::alga2::op::Additive, ::alga2::op::Multiplicative> for #n #ty_g
            where #where_g #(#semiring_b),*
            {}
            impl #impl_g ::alga2::tower::Ring<::alga2::op::Additive, ::alga2::op::Multiplicative> for #n #ty_g
            where #where_g #(#semiring_b),*
            {}
            impl #impl_g ::alga2::tower::CommutativeRing<::alga2::op::Additive, ::alga2::op::Multiplicative> for #n #ty_g
            where #where_g #(#comm_b),*
            {}
            #field_ladder
        }
    }

    /// Module / VectorSpace: scalar from the first field, scale component-wise.
    fn module_level(&self, fields: &[FieldInfo]) -> TS {
        let n = self.name;
        let impl_g = self.impl_g;
        let ty_g = self.ty_g;
        let where_g = self.where_g;
        let first = fields.first().unwrap();

        let module_b = field_types(fields).iter().map(|ty| {
            quote!(#ty: ::alga2::tower::Module<::alga2::op::Additive, ::alga2::op::Multiplicative>)
        }).collect::<Vec<_>>();

        let scale_exprs = fields.iter().map(|f| {
            let s_ = &f.suffix;
            let a = quote!(v.#s_);
            quote!(::alga2::tower::Module::<::alga2::op::Additive, ::alga2::op::Multiplicative>::scale(s, #a))
        }).collect::<Vec<_>>();
        let c_scale = construct(fields, &scale_exprs);

        let ft = &first.ty;
        quote! {
            impl #impl_g ::alga2::tower::Module<::alga2::op::Additive, ::alga2::op::Multiplicative> for #n #ty_g
            where #where_g #(#module_b),*
            {
                type Scalar = <#ft as ::alga2::tower::Module<::alga2::op::Additive, ::alga2::op::Multiplicative>>::Scalar;
                fn scale(s: &Self::Scalar, v: Self) -> Self { #c_scale }
            }
        }
    }
}
