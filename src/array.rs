//! This module provides a type-level array of type-level numbers.
//!
//! It is not very featureful right now, and should be considered a work in progress.

use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

use super::*;

/// The terminating type for type arrays.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct ATerm;

impl TypeArray for ATerm {}

/// `TArr` is a type that acts as an array of types. It is defined similarly to `UInt`, only its
/// values can be more than bits, and it is designed to act as an array. So you can only add two if
/// they have the same number of elements, for example.
///
/// This array is only really designed to contain `Integer` types. If you use it with others, you
/// may find it lacking functionality.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct TArr<V, A> {
    _marker: PhantomData<(V, A)>,
}

impl<V, A> TypeArray for TArr<V, A> {}

/// Create a new type-level arrray. Only usable on Rust 1.13.0 or newer.
///
/// There's not a whole lot you can do with it right now.
///
/// # Example
/// ```ignore
/// #[macro_use]
/// extern crate typenum;
/// use typenum::consts::*;
///
/// type Array = tarr![P3, N4, Z0, P38];
/// # fn main() {}
#[macro_export]
macro_rules! tarr {
    () => ( $crate::ATerm );
    ($n:ty) => ( $crate::TArr<$n, $crate::ATerm> );
    ($n:ty,) => ( $crate::TArr<$n, $crate::ATerm> );
    ($n:ty, $($tail:ty),+) => ( $crate::TArr<$n, tarr![$($tail),+]> );
    ($n:ty, $($tail:ty),+,) => ( $crate::TArr<$n, tarr![$($tail),+]> );
}

// ---------------------------------------------------------------------------------------
// Length

/// Length of `ATerm` by itself is 0
impl Len for ATerm {
    type Output = U0;
}

/// Size of a `TypeArray`
impl<V, A> Len for TArr<V, A>
    where A: Len,
          Length<A>: Add<B1>,
          Sum<Length<A>, B1>: Unsigned
{
    type Output = Add1<Length<A>>;
}

// ---------------------------------------------------------------------------------------
// Add arrays
// Note that two arrays are only addable if they are the same length.

impl Add<ATerm> for ATerm {
    type Output = ATerm;
    fn add(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<Al, Vl, Ar, Vr> Add<TArr<Vr, Ar>> for TArr<Vl, Al>
    where Al: Add<Ar>,
          Vl: Add<Vr>
{
    type Output = TArr<Sum<Vl, Vr>, Sum<Al, Ar>>;
    fn add(self, _: TArr<Vr, Ar>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Subtract arrays
// Note that two arrays are only subtractable if they are the same length.

impl Sub<ATerm> for ATerm {
    type Output = ATerm;
    fn sub(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<Vl, Al, Vr, Ar> Sub<TArr<Vr, Ar>> for TArr<Vl, Al>
    where Vl: Sub<Vr>,
          Al: Sub<Ar>
{
    type Output = TArr<Diff<Vl, Vr>, Diff<Al, Ar>>;
    fn sub(self, _: TArr<Vr, Ar>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Multiply an array by a scalar

impl<Rhs> Mul<Rhs> for ATerm {
    type Output = ATerm;
    fn mul(self, _: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, Rhs> Mul<Rhs> for TArr<V, A>
    where V: Mul<Rhs>,
          A: Mul<Rhs>
{
    type Output = TArr<Prod<V, Rhs>, Prod<A, Rhs>>;
    fn mul(self, _: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl Mul<ATerm> for Z0 {
    type Output = ATerm;
    fn mul(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<U> Mul<ATerm> for PInt<U>
    where U: Unsigned + NonZero
{
    type Output = ATerm;
    fn mul(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<U> Mul<ATerm> for NInt<U>
    where U: Unsigned + NonZero
{
    type Output = ATerm;
    fn mul(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<V, A> Mul<TArr<V, A>> for Z0
    where Z0: Mul<A>
{
    type Output = TArr<Z0, Prod<Z0, A>>;
    fn mul(self, _: TArr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Mul<TArr<V, A>> for PInt<U>
    where U: Unsigned + NonZero,
          PInt<U>: Mul<A> + Mul<V>
{
    type Output = TArr<Prod<PInt<U>, V>, Prod<PInt<U>, A>>;
    fn mul(self, _: TArr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Mul<TArr<V, A>> for NInt<U>
    where U: Unsigned + NonZero,
          NInt<U>: Mul<A> + Mul<V>
{
    type Output = TArr<Prod<NInt<U>, V>, Prod<NInt<U>, A>>;
    fn mul(self, _: TArr<V, A>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Divide an array by a scalar

impl<Rhs> Div<Rhs> for ATerm {
    type Output = ATerm;
    fn div(self, _: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, Rhs> Div<Rhs> for TArr<V, A>
    where V: Div<Rhs>,
          A: Div<Rhs>
{
    type Output = TArr<Quot<V, Rhs>, Quot<A, Rhs>>;
    fn div(self, _: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl Div<ATerm> for Z0 {
    type Output = ATerm;
    fn div(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<U> Div<ATerm> for PInt<U>
    where U: Unsigned + NonZero
{
    type Output = ATerm;
    fn div(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<U> Div<ATerm> for NInt<U>
    where U: Unsigned + NonZero
{
    type Output = ATerm;
    fn div(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<V, A> Div<TArr<V, A>> for Z0
    where Z0: Div<A>
{
    type Output = TArr<Z0, Quot<Z0, A>>;
    fn div(self, _: TArr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Div<TArr<V, A>> for PInt<U>
    where U: Unsigned + NonZero,
          PInt<U>: Div<A> + Div<V>
{
    type Output = TArr<Quot<PInt<U>, V>, Quot<PInt<U>, A>>;
    fn div(self, _: TArr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Div<TArr<V, A>> for NInt<U>
    where U: Unsigned + NonZero,
          NInt<U>: Div<A> + Div<V>
{
    type Output = TArr<Quot<NInt<U>, V>, Quot<NInt<U>, A>>;
    fn div(self, _: TArr<V, A>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
