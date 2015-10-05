
use std::marker::PhantomData;

use ::{Same};
use ::bit::{Bit};
use ::uint::{Unsigned};

/// This trait is implemented for the all things that an `Int` can take as a parameter,
/// which is just `Int` and `ITerm` (used to terminate the `Int`). It should not be
/// implemented for anything outside this crate.
pub trait Signed {
    fn to_int() -> i64;
}

/// A type representing a positive flag
pub struct Positive;
/// A type representing a negative flag
pub struct Negative;

/// A trait for `Positive` and `Negative`, to represent the sign of an integer. It
/// should not be implemented for anything outside this crate.
pub trait Sign {}
impl Sign for Positive {}
impl Sign for Negative {}

/// `Int` is defined as a `UInt` with a sign flag.
pub struct Int<U: Unsigned, S: Sign> {
    _marker: PhantomData<(U, S)>
}

impl<U: Unsigned> Signed for Int<U, Positive> {
    fn to_int() -> i64 {
        U::to_int() as i64
    }
}
impl<U: Unsigned> Signed for Int<U, Negative> {
    fn to_int() -> i64 {
        - (U::to_int() as i64)
    }
}

impl<U: Unsigned, S: Sign> Same<Int<U, S>> for Int<U, S> {
    type Output = Int<U, S>;
}
