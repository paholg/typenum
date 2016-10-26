//! Docs go here

use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

use super::*;
// pub trait TypeArray { }

/// The terminating type for the type array `Array`.
pub enum ATerm {}

/// `Array` is a type that acts as an array of types. It is defined similarly to `UInt`, only its
/// values can be more than bits, and it is designed to act as an array. So you can only add two if
/// they have the same number of elements, for example.
///
/// This array is only really designed to contain `Integer` types. If you use it with others, you
/// may find it lacking functionality.
pub struct Array<A, V> {
    _marker: PhantomData<(A, V)>,
}

// ---------------------------------------------------------------------------------------
// Array length!

/// Length of `ATerm` by itself is 0
impl Len for ATerm {
    type Output = U0;
}

/// Size of an `Array`
impl<A, V> Len for Array<A, V>
    where A: Len,
          Length<A>: Add<B1>,
          Sum<Length<A>, B1>: Unsigned,
{
    type Output = Sum<Length<A>, B1>;
}

// ---------------------------------------------------------------------------------------
// Push to array

/// Pushes `V` to the end of an `Array`
pub trait Push<V> {
    /// The new, longer array
    type Output;
}

pub type Pushed<A, V> = <A as Push<V>>::Output;

impl<V> Push<V> for ATerm {
    type Output = Array<ATerm, V>;
}

impl<V, A, V2> Push<V> for Array<A, V2> {
    type Output = Array<Array<ATerm, V2>, V>;
}
// ---------------------------------------------------------------------------------------
// Pop from array

/// Pops the last element off an `Array`
pub trait Pop {
    /// The new, shorter array
    type Output;
}

pub type Popped<A> = <A as Pop>::Output;

impl<A, V> Pop for Array<A, V> {
    type Output = A;
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

impl<Al, Vl, Ar, Vr> Add<Array<Ar, Vr>> for Array<Al, Vl> where
    Al: Add<Ar>,
    Vl: Add<Vr>,
{
    type Output = Array<Sum<Al, Ar>, Sum<Vl, Vr>>;
    fn add(self, _: Array<Ar, Vr>) -> Self::Output {
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

impl<Al, Vl, Ar, Vr> Sub<Array<Ar, Vr>> for Array<Al, Vl> where
    Al: Sub<Ar>,
    Vl: Sub<Vr>,
{
    type Output = Array<Diff<Al, Ar>, Diff<Vl, Vr>>;
    fn sub(self, _: Array<Ar, Vr>) -> Self::Output {
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

impl<A, V, Rhs> Mul<Rhs> for Array<A, V> where V: Mul<Rhs>, A: Mul<Rhs> {
    type Output = Array<Prod<A, Rhs>, Prod<V, Rhs>>;
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

impl<U> Mul<ATerm> for PInt<U> where U: Unsigned + NonZero {
    type Output = ATerm;
    fn mul(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<U> Mul<ATerm> for NInt<U> where U: Unsigned + NonZero {
    type Output = ATerm;
    fn mul(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<A, V> Mul<Array<A, V>> for Z0 where Z0: Mul<A> {
    type Output = Array<Prod<Z0, A>, Z0>;
    fn mul(self, _: Array<A, V>) -> Self::Output {
        unreachable!()
    }
}

impl<A, V, U> Mul<Array<A, V>> for PInt<U> where U: Unsigned + NonZero, PInt<U>: Mul<A> + Mul<V> {
    type Output = Array<Prod<PInt<U>, A>, Prod<PInt<U>, V>>;
    fn mul(self, _: Array<A, V>) -> Self::Output {
        unreachable!()
    }
}

impl<A, V, U> Mul<Array<A, V>> for NInt<U> where U: Unsigned + NonZero, NInt<U>: Mul<A> + Mul<V> {
    type Output = Array<Prod<NInt<U>, A>, Prod<NInt<U>, V>>;
    fn mul(self, _: Array<A, V>) -> Self::Output {
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

impl<A, V, Rhs> Div<Rhs> for Array<A, V> where V: Div<Rhs>, A: Div<Rhs> {
    type Output = Array<Quot<A, Rhs>, Quot<V, Rhs>>;
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

impl<U> Div<ATerm> for PInt<U> where U: Unsigned + NonZero {
    type Output = ATerm;
    fn div(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<U> Div<ATerm> for NInt<U> where U: Unsigned + NonZero {
    type Output = ATerm;
    fn div(self, _: ATerm) -> Self::Output {
        unreachable!()
    }
}

impl<A, V> Div<Array<A, V>> for Z0 where Z0: Div<A> {
    type Output = Array<Quot<Z0, A>, Z0>;
    fn div(self, _: Array<A, V>) -> Self::Output {
        unreachable!()
    }
}

impl<A, V, U> Div<Array<A, V>> for PInt<U> where U: Unsigned + NonZero, PInt<U>: Div<A> + Div<V> {
    type Output = Array<Quot<PInt<U>, A>, Quot<PInt<U>, V>>;
    fn div(self, _: Array<A, V>) -> Self::Output {
        unreachable!()
    }
}

impl<A, V, U> Div<Array<A, V>> for NInt<U> where U: Unsigned + NonZero, NInt<U>: Div<A> + Div<V> {
    type Output = Array<Quot<NInt<U>, A>, Quot<NInt<U>, V>>;
    fn div(self, _: Array<A, V>) -> Self::Output {
        unreachable!()
    }
}

