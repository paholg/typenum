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
pub trait Pow<Exp> {
    /// The result of the exponentiation.
    type Output;
    /// This function isn't used in this crate, but may be useful for others.
    /// It is implemented for primitives.
    ///
    /// # Example
    /// ```rust
    /// use typenum::{Pow, U3};
    ///
    /// let a = 7u32.powi(U3::new());
    /// let b = 7u32.pow(3);
    /// assert_eq!(a, b);
    ///
    /// let x = 3.0.powi(U3::new());
    /// let y = 27.0;
    /// assert_eq!(x, y);
    /// ```
    fn powi(self, exp: Exp) -> Self::Output;
}

use {Unsigned, Bit, UInt, PInt, NonZero, UTerm, Z0};
macro_rules! impl_pow_f {
    ($t: ty) => (
        impl Pow<UTerm> for $t {
            type Output = $t;
            #[inline]
            fn powi(self, _: UTerm) -> Self::Output {
                1.0
            }
        }

        impl<U: Unsigned, B: Bit> Pow<UInt<U, B>> for $t {
            type Output = $t;
            // powi is unstable in core, so we have to write this function ourselves.
            // copied from num::pow::pow
            #[inline]
            fn powi(self, _: UInt<U, B>) -> Self::Output {
                let mut exp = <UInt<U, B> as Unsigned>::to_u32();
                let mut base = self;

                if exp == 0 { return 1.0 }

                while exp & 1 == 0 {
                    base *= base;
                    exp >>= 1;
                }
                if exp == 1 { return base }

                let mut acc = base.clone();
                while exp > 1 {
                    exp >>= 1;
                    base *= base;
                    if exp & 1 == 1 {
                        acc *= base.clone();
                    }
                }
                acc
            }
        }

        impl Pow<Z0> for $t {
            type Output = $t;
            #[inline]
            fn powi(self, _: Z0) -> Self::Output {
                1.0
            }
        }

        impl<U: Unsigned + NonZero> Pow<PInt<U>> for $t {
            type Output = $t;
            // powi is unstable in core, so we have to write this function ourselves.
            // copied from num::pow::pow
            #[inline]
            fn powi(self, _: PInt<U>) -> Self::Output {
                let mut exp = U::to_u32();
                let mut base = self;

                if exp == 0 { return 1.0 }

                while exp & 1 == 0 {
                    base *= base;
                    exp >>= 1;
                }
                if exp == 1 { return base }

                let mut acc = base.clone();
                while exp > 1 {
                    exp >>= 1;
                    base *= base;
                    if exp & 1 == 1 {
                        acc *= base.clone();
                    }
                }
                acc
            }
        }
    );
}

impl_pow_f!(f32);
impl_pow_f!(f64);


macro_rules! impl_pow_i {
    () => ();
    ($t: ty $(, $tail:tt)*) => (
        impl Pow<UTerm> for $t {
            type Output = $t;
            #[inline]
            fn powi(self, _: UTerm) -> Self::Output {
                1
            }
        }

        impl<U: Unsigned, B: Bit> Pow<UInt<U, B>> for $t {
            type Output = $t;
            #[inline]
            fn powi(self, _: UInt<U, B>) -> Self::Output {
                self.pow(<UInt<U, B> as Unsigned>::to_u32())
            }
        }

        impl Pow<Z0> for $t {
            type Output = $t;
            #[inline]
            fn powi(self, _: Z0) -> Self::Output {
                1
            }
        }

        impl<U: Unsigned + NonZero> Pow<PInt<U>> for $t {
            type Output = $t;
            #[inline]
            fn powi(self, _: PInt<U>) -> Self::Output {
                self.pow(U::to_u32())
            }
        }

        impl_pow_i!($($tail),*);
    );
}

impl_pow_i!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

#[test]
fn pow_test() {
    use consts::*;
    let z0 = Z0::new();
    let p3 = P3::new();

    let u0 = U0::new();
    let u3 = U3::new();

    macro_rules! check {
        ($x:ident) => (
            assert_eq!($x.powi(z0), 1);
            assert_eq!($x.powi(u0), 1);

            assert_eq!($x.powi(p3), $x*$x*$x);
            assert_eq!($x.powi(u3), $x*$x*$x);
        );
        ($x:ident, $f:ident) => (
            assert!((<$f as Pow<Z0>>::powi(*$x, z0) - 1.0).abs() < ::core::$f::EPSILON);
            assert!((<$f as Pow<U0>>::powi(*$x, u0) - 1.0).abs() < ::core::$f::EPSILON);

            assert!((<$f as Pow<P3>>::powi(*$x, p3) - $x*$x*$x).abs() < ::core::$f::EPSILON);
            assert!((<$f as Pow<U3>>::powi(*$x, u3) - $x*$x*$x).abs() < ::core::$f::EPSILON);
        );
    }

    for x in &[0i8, -3, 2] {
        check!(x);
    }
    for x in &[0u8, 1, 5] {
        check!(x);
    }
    for x in &[0usize, 1, 5, 40] {
        check!(x);
    }
    for x in &[0isize, 1, 2, -30, -22, 48] {
        check!(x);
    }
    for x in &[0.0f32, 2.2, -3.5, 378.223] {
        check!(x, f32);
    }
    for x in &[0.0f64, 2.2, -3.5, -2387.2, 234.22] {
        check!(x, f64);
    }
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

/// A **type operator** that gives the length of an `Array` or the number of bits in a `UInt`.
pub trait Len {
    /// The length as a type-level unsigned integer.
    type Output: ::Unsigned;
    /// This function isn't used in this crate, but may be useful for others.
    fn len(&self) -> Self::Output;
}
