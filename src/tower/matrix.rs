//! Matrices as an abstract interface (alga-aligned).
//!
//! These traits describe what a matrix *is* in terms of the tower — a finite
//! `Field`-valued rectangular array with row/column vector spaces — without
//! pinning a representation. Downstream linear-algebra types (a 2×2 tuple
//! matrix, a nalgebra-style `Matrix`, ...) implement them; the tower supplies
//! the vocabulary (`Field` cells, `FiniteDimVectorSpace` rows/columns).

use crate::op::{Additive, Multiplicative};
use crate::tower::{Field, FiniteDimVectorSpace};

/// A rectangular matrix: a `Field`-valued array with row/column spaces.
pub trait Matrix: Sized + Clone {
    /// The scalar field of the entries.
    type Field: Field<Additive, Multiplicative> + Copy;

    /// The row space (a finite-dimensional vector space over the field).
    type Row: FiniteDimVectorSpace<Additive, Multiplicative, Scalar = Self::Field>;

    /// The column space.
    type Column: FiniteDimVectorSpace<Additive, Multiplicative, Scalar = Self::Field>;

    /// The transposed matrix type.
    type Transpose: Matrix<Field = Self::Field, Row = Self::Column, Column = Self::Row>;

    /// The number of rows.
    fn nrows(&self) -> usize;

    /// The number of columns.
    fn ncolumns(&self) -> usize;

    /// The `i`-th row.
    fn row(&self, i: usize) -> Self::Row;

    /// The `j`-th column.
    fn column(&self, j: usize) -> Self::Column;

    /// The entry at `(i, j)`.
    fn get(&self, i: usize, j: usize) -> Self::Field;

    /// The transpose.
    fn transpose(&self) -> Self::Transpose;
}

/// A mutable matrix.
pub trait MatrixMut: Matrix {
    /// Sets the `i`-th row, returning the modified matrix.
    fn set_row(&self, i: usize, row: &Self::Row) -> Self;

    /// Sets the `i`-th row in place.
    fn set_row_mut(&mut self, i: usize, row: &Self::Row);

    /// Sets the `j`-th column, returning the modified matrix.
    fn set_column(&self, j: usize, column: &Self::Column) -> Self;

    /// Sets the `j`-th column in place.
    fn set_column_mut(&mut self, j: usize, column: &Self::Column);

    /// Sets the entry at `(i, j)` in place.
    fn set(&mut self, i: usize, j: usize, val: Self::Field);
}

/// A square matrix: diagonal, determinant, inverse.
pub trait SquareMatrix: Matrix {
    /// The diagonal vector type (a finite-dimensional space over the field).
    type Vector: FiniteDimVectorSpace<Additive, Multiplicative, Scalar = Self::Field>;

    /// The dimension (`nrows == ncolumns`).
    fn dimension(&self) -> usize;

    /// The diagonal vector.
    fn diagonal(&self) -> Self::Vector;

    /// The determinant.
    fn determinant(&self) -> Self::Field;

    /// The inverse, `None` for singular matrices.
    fn try_inverse(&self) -> Option<Self>;
}

/// A mutable square matrix.
pub trait SquareMatrixMut: SquareMatrix + MatrixMut {
    /// Transposes in place.
    fn transpose_mut(&mut self);
}

/// An invertible square matrix: [`SquareMatrix::try_inverse`] is total.
pub trait InversibleSquareMatrix: SquareMatrix {
    /// The inverse (total — the matrix is guaranteed invertible).
    fn inverse(&self) -> Self;
}
