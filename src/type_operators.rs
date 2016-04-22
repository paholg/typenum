//! This module provides useful **type operators** that are not defined in `core`.
//!
//!

/// A **type operator** that ensures that `Rhs` is the same as `Self`, it is mainly useful
/// for writing macros that can take arbitrary binary or unary operators.
///
/// `Same` is implemented generically for all types; it should never need to be implemented
/// for anything else.
///
/// Note that Rust lazily evaluates types, so this will only fail for two different types if
/// the `Output` is used.
///
/// # Example
/// ```rust
/// use typenum::{Same, U4, U5, Unsigned};
///
/// assert_eq!(<U5 as Same<U5>>::Output::to_u32(), 5);
///
/// // Only an error if we use it:
/// type Undefined = <U5 as Same<U4>>::Output;
/// // Compiler error:
/// // Undefined::to_u32();
/// ```
pub trait Same<Rhs = Self> {
    /// Should always be `Self`
    type Output;
}

impl<T> Same<T> for T {
    type Output = T;
}

/// A **type operator** that provides exponentiation by repeated squaring.
///
/// # Example
/// ```rust
/// use typenum::{Pow, N3, P3, Integer};
///
/// assert_eq!(<N3 as Pow<P3>>::Output::to_i32(), -27);
/// ```
pub trait Pow<Rhs = Self> {
    /// The result of the exponentiation.
    type Output;
}

/// A **type operator** for comparing `Self` and `Rhs`. It provides a similar functionality to
/// the function
/// [`core::cmp::Ord::cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)
/// but for types.
///
/// # Example
/// ```rust
/// use typenum::{Cmp, Ord, Greater, Less, Equal, N3, P2, P5};
/// use std::cmp::Ordering;
///
/// assert_eq!(<P2 as Cmp<N3>>::Output::to_ordering(), Ordering::Greater);
/// assert_eq!(<P2 as Cmp<P2>>::Output::to_ordering(), Ordering::Equal);
/// assert_eq!(<P2 as Cmp<P5>>::Output::to_ordering(), Ordering::Less);
pub trait Cmp<Rhs = Self> {
    /// The result of the comparison. It should only ever be one of `Greater`, `Less`, or `Equal`.
    type Output;
}
