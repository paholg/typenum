//! Docs go here

use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

use super::*;
pub trait Array { }

/// The terminating type for the type array `Array`.
pub enum ATerm {}
impl_derivable!(ATerm);

impl Array for ATerm {}

/// `Arr` is a type that acts as an array of types. It is defined similarly to `UInt`, only its
/// values can be more than bits, and it is designed to act as an array. So you can only add two if
/// they have the same number of elements, for example.
///
/// This array is only really designed to contain `Integer` types. If you use it with others, you
/// may find it lacking functionality.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct Arr<V, A> {
    _marker: PhantomData<(V, A)>,
}

impl<V, A> Array for Arr<V, A> {}

#[macro_export]
macro_rules! array {
    () => ( $crate::ATerm );
    ($n:ty) => ( $crate::Arr<$n, $crate::ATerm> );
    ($n:ty,) => ( $crate::Arr<$n, $crate::ATerm> );
    ($n:ty, $($tail:ty),+) => ( $crate::Arr<$n, array![$($tail),+]> );
    ($n:ty, $($tail:ty),+,) => ( $crate::Arr<$n, array![$($tail),+]> );
}

// ---------------------------------------------------------------------------------------
// Length

/// Length of `ATerm` by itself is 0
impl Len for ATerm {
    type Output = U0;
}

/// Size of an `Array`
impl<V, A> Len for Arr<V, A>
    where A: Len,
          Length<A>: Add<B1>,
          Sum<Length<A>, B1>: Unsigned,
{
    type Output = Add1<Length<A>>;
}

// ---------------------------------------------------------------------------------------
// Push to array

/// Pushes `V` to the end of an `Arr`
pub trait Push<V> {
    /// The new, longer array
    type Output;
}

pub type Pushed<V, A> = <A as Push<V>>::Output;

impl<V> Push<V> for ATerm {
    type Output = Arr<V, ATerm>;
}

impl<V, A, V2> Push<V> for Arr<V2, A> {
    type Output = Arr<V, Arr<V2, ATerm>>;
}

// ---------------------------------------------------------------------------------------
// Pop from array

/// Pops the last element off an `Arr`
pub trait Pop {
    /// The new, shorter array
    type Array;
    /// The last element
    type Last;
}

pub type Popped<A> = <A as Pop>::Array;
pub type Last<A> = <A as Pop>::Last;

impl<V, A> Pop for Arr<V, A> {
    type Array = A;
    type Last = V;
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

impl<Al, Vl, Ar, Vr> Add<Arr<Vr, Ar>> for Arr<Vl, Al> where Al: Add<Ar>, Vl: Add<Vr>, {
    type Output = Arr<Sum<Vl, Vr>, Sum<Al, Ar>>;
    fn add(self, _: Arr<Vr, Ar>) -> Self::Output {
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

impl<Vl, Al, Vr, Ar> Sub<Arr<Vr, Ar>> for Arr<Vl, Al> where Vl: Sub<Vr>, Al: Sub<Ar>, {
    type Output = Arr<Diff<Vl, Vr>, Diff<Al, Ar>>;
    fn sub(self, _: Arr<Vr, Ar>) -> Self::Output {
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

impl<V, A, Rhs> Mul<Rhs> for Arr<V, A> where V: Mul<Rhs>, A: Mul<Rhs> {
    type Output = Arr<Prod<V, Rhs>, Prod<A, Rhs>>;
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

impl<V, A> Mul<Arr<V, A>> for Z0 where Z0: Mul<A> {
    type Output = Arr<Z0, Prod<Z0, A>>;
    fn mul(self, _: Arr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Mul<Arr<V, A>> for PInt<U> where U: Unsigned + NonZero, PInt<U>: Mul<A> + Mul<V> {
    type Output = Arr<Prod<PInt<U>, V>, Prod<PInt<U>, A>>;
    fn mul(self, _: Arr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Mul<Arr<V, A>> for NInt<U> where U: Unsigned + NonZero, NInt<U>: Mul<A> + Mul<V> {
    type Output = Arr<Prod<NInt<U>, V>, Prod<NInt<U>, A>>;
    fn mul(self, _: Arr<V, A>) -> Self::Output {
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

impl<V, A, Rhs> Div<Rhs> for Arr<V, A> where V: Div<Rhs>, A: Div<Rhs> {
    type Output = Arr<Quot<V, Rhs>, Quot<A, Rhs>>;
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

impl<V, A> Div<Arr<V, A>> for Z0 where Z0: Div<A> {
    type Output = Arr<Z0, Quot<Z0, A>>;
    fn div(self, _: Arr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Div<Arr<V, A>> for PInt<U> where U: Unsigned + NonZero, PInt<U>: Div<A> + Div<V> {
    type Output = Arr<Quot<PInt<U>, V>, Quot<PInt<U>, A>>;
    fn div(self, _: Arr<V, A>) -> Self::Output {
        unreachable!()
    }
}

impl<V, A, U> Div<Arr<V, A>> for NInt<U> where U: Unsigned + NonZero, NInt<U>: Div<A> + Div<V> {
    type Output = Arr<Quot<NInt<U>, V>, Quot<NInt<U>, A>>;
    fn div(self, _: Arr<V, A>) -> Self::Output {
        unreachable!()
    }
}

