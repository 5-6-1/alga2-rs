//! Operator-closure markers: the type is closed under a std operator.
//!
//! These are the "the operation never leaves the type" certificates that
//! downstream generic code (e.g. vector/geometry algorithms) can bound on.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Closed under `+`/`+=`.
pub trait ClosedAdd<Right = Self>: Sized + Add<Right, Output = Self> + AddAssign<Right> {}

/// Closed under `-`/`-=`.
pub trait ClosedSub<Right = Self>: Sized + Sub<Right, Output = Self> + SubAssign<Right> {}

/// Closed under `*`/`*=`.
pub trait ClosedMul<Right = Self>: Sized + Mul<Right, Output = Self> + MulAssign<Right> {}

/// Closed under `/`/`/=`.
pub trait ClosedDiv<Right = Self>: Sized + Div<Right, Output = Self> + DivAssign<Right> {}

/// Closed under unary `-`.
pub trait ClosedNeg: Sized + Neg<Output = Self> {}
