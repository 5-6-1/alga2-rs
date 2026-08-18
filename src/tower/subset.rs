//! Numeric subset/superset embedding.
//!
//! `SubsetOf<T>` says `Self` embeds into `T` and `T`'s values in the image
//! round-trip. For the lossless chains (`u8 ⊂ u16 ⊂ …`) every `T` value in
//! range maps back; a lossy embedding (`u8 ⊂ f64`) still carries the
//! mathematical subset relation but `from_superset` rejects values that
//! cannot be represented exactly.

/// `Self` is a subset of `T`: every value of `Self` maps losslessly into
/// `T`, and the `T`-values in the embedded copy round-trip.
pub trait SubsetOf<T>: Sized {
    /// The canonical embedding `Self → T`.
    fn to_superset(&self) -> T;

    /// Whether `element` lies in the embedded copy of `Self` (defaults to
    /// `true` — the lossless embedding case).
    fn is_in_subset(element: &T) -> bool {
        let _ = element;
        true
    }

    /// The inverse of `to_superset`; `None` outside the embedded copy.
    fn from_superset(element: &T) -> Option<Self> {
        if Self::is_in_subset(element) {
            Some(Self::from_superset_unchecked(element))
        } else {
            None
        }
    }

    /// The inverse of `to_superset` without the subset check — only valid
    /// when `is_in_subset` holds.
    fn from_superset_unchecked(element: &T) -> Self;
}

/// `Self` is a superset of `T`: the dual of [`SubsetOf`], automatically
/// provided whenever `T: SubsetOf<Self>`.
pub trait SupersetOf<T>: Sized {
    /// The canonical projection `Self → T`; `None` outside the subset.
    fn to_subset(&self) -> Option<T>;

    /// The inverse of `to_subset` (total: every `T` value embeds).
    fn from_subset(element: &T) -> Self;

    /// Whether `self` lies in the embedded copy of `T`.
    fn is_in_subset(&self) -> bool;
}

impl<SS, SP> SupersetOf<SS> for SP
where
    SS: SubsetOf<SP>,
{
    fn to_subset(&self) -> Option<SS> {
        SS::from_superset(self)
    }

    fn from_subset(element: &SS) -> Self {
        element.to_superset()
    }

    fn is_in_subset(&self) -> bool {
        SS::is_in_subset(self)
    }
}
