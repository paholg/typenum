
use std::marker::PhantomData;

use std::ops::{Neg, Add, Sub, Mul, Div, Pow};
use {NonZero, Same, Cmp, Greater, Equal, Less};
use uint::{Unsigned};
use __private::{PrivateIntegerAdd, PrivateDivFirstStep};
use consts::U1;

/// Positive integers
pub struct PInt<U: Unsigned + NonZero> {
    _marker: PhantomData<U>
}
/// Negative integers
pub struct NInt<U: Unsigned + NonZero> {
    _marker: PhantomData<U>
}
/// The signed integer 0
pub struct Z0;

pub trait Integer {
    fn to_i8() -> i8;
    fn to_i16() -> i16;
    fn to_i32() -> i32;
    fn to_i64() -> i64;
    fn to_isize() -> isize;
}

impl<U: Unsigned + NonZero> NonZero for PInt<U> {}
impl<U: Unsigned + NonZero> NonZero for NInt<U> {}

impl Integer for Z0 {
    fn to_i8() -> i8 { 0 }
    fn to_i16() -> i16 { 0 }
    fn to_i32() -> i32 { 0 }
    fn to_i64() -> i64 { 0 }
    fn to_isize() -> isize { 0 }
}

impl<U: Unsigned + NonZero> Integer for PInt<U> {
    fn to_i8() -> i8 { <U as Unsigned>::to_i8() }
    fn to_i16() -> i16 { <U as Unsigned>::to_i16() }
    fn to_i32() -> i32 { <U as Unsigned>::to_i32() }
    fn to_i64() -> i64 { <U as Unsigned>::to_i64() }
    fn to_isize() -> isize { <U as Unsigned>::to_isize() }
}

impl<U: Unsigned + NonZero> Integer for NInt<U> {
    fn to_i8() -> i8 { -<U as Unsigned>::to_i8() }
    fn to_i16() -> i16 { -<U as Unsigned>::to_i16() }
    fn to_i32() -> i32 { -<U as Unsigned>::to_i32() }
    fn to_i64() -> i64 { -<U as Unsigned>::to_i64() }
    fn to_isize() -> isize { -<U as Unsigned>::to_isize() }
}

impl Same<Z0> for Z0 {
    type Output = Z0;
}

impl<U: Unsigned + NonZero> Same<PInt<U>> for PInt<U> {
    type Output = PInt<U>;
}

impl<U: Unsigned + NonZero> Same<NInt<U>> for NInt<U> {
    type Output = NInt<U>;
}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_int_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Integer>::to_i64(), <Test as Integer>::to_i64());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Integer>::to_i64(), <Test as Integer>::to_i64());
        }
        );
}

// ---------------------------------------------------------------------------------------
// Neg

/// `-Z0 = Z0`
impl Neg for Z0 {
    type Output = Z0;
    fn neg(self) -> Self::Output { unreachable!() }
}

/// `-PInt = NInt`
impl<U: Unsigned + NonZero> Neg for PInt<U> {
    type Output = NInt<U>;
    fn neg(self) -> Self::Output { unreachable!() }
}

/// `-NInt = PInt`
impl<U: Unsigned + NonZero> Neg for NInt<U> {
    type Output = PInt<U>;
    fn neg(self) -> Self::Output { unreachable!() }
}

// ---------------------------------------------------------------------------------------
// Add

/// `Z0 + I = I`
impl<I: Integer> Add<I> for Z0 {
    type Output = I;
    fn add(self, _: I) -> Self::Output { unreachable!() }
}

/// `PInt + Z0 = PInt`
impl<U: Unsigned + NonZero> Add<Z0> for PInt<U> {
    type Output = PInt<U>;
    fn add(self, _: Z0) -> Self::Output { unreachable!() }
}

/// `NInt + Z0 = NInt`
impl<U: Unsigned + NonZero> Add<Z0> for NInt<U> {
    type Output = NInt<U>;
    fn add(self, _: Z0) -> Self::Output { unreachable!() }
}

/// `P(Ul) + P(Ur) = P(Ul + Ur)`
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Add<PInt<Ur>> for PInt<Ul>
    where Ul: Add<Ur>,
          <Ul as Add<Ur>>::Output: Unsigned + NonZero
{
    type Output = PInt<<Ul as Add<Ur>>::Output>;
    fn add(self, _: PInt<Ur>) -> Self::Output { unreachable!() }
}

/// `N(Ul) + N(Ur) = N(Ul + Ur)`
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Add<NInt<Ur>> for NInt<Ul>
    where Ul: Add<Ur>,
          <Ul as Add<Ur>>::Output: Unsigned + NonZero
{
    type Output = NInt<<Ul as Add<Ur>>::Output>;
    fn add(self, _: NInt<Ur>) -> Self::Output { unreachable!() }
}

/// `P(Ul) + N(Ur)`: We resolve this with our `PrivateAdd`
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Add<NInt<Ur>> for PInt<Ul>
    where Ul: Cmp<Ur> + PrivateIntegerAdd<<Ul as Cmp<Ur>>::Output, Ur>
{
    type Output = <Ul as PrivateIntegerAdd<
        <Ul as Cmp<Ur>>::Output, Ur
        >>::Output;
    fn add(self, _: NInt<Ur>) -> Self::Output { unreachable!() }
}

/// `P(Ul) + P(Ur)`: We resolve this with our `PrivateAdd`
// We just do the same thing as above, swapping Lhs and Rhs
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Add<PInt<Ur>> for NInt<Ul>
    where Ur: Cmp<Ul> + PrivateIntegerAdd<<Ur as Cmp<Ul>>::Output, Ul>
{
    type Output = <Ur as PrivateIntegerAdd<
        <Ur as Cmp<Ul>>::Output, Ul
        >>::Output;
    fn add(self, _: PInt<Ur>) -> Self::Output { unreachable!() }
}

/// `P + N = 0` where `P == N`
impl<N: Unsigned, P: Unsigned> PrivateIntegerAdd<Equal, N> for P {
    type Output = Z0;
}

/// `P + N = Positive` where `P > N`
impl<N: Unsigned, P: Unsigned> PrivateIntegerAdd<Greater, N> for P
    where P: Sub<N>,
          <P as Sub<N>>::Output: Unsigned + NonZero
{
    type Output = PInt<<P as Sub<N>>::Output>;
}

/// `P + N = Negative` where `P < N`
impl<N: Unsigned, P: Unsigned> PrivateIntegerAdd<Less, N> for P
    where N: Sub<P>,
          <N as Sub<P>>::Output: Unsigned + NonZero
{
    type Output = NInt<<N as Sub<P>>::Output>;
}

// ---------------------------------------------------------------------------------------
// Sub

/// `Z0 - Z0 = Z0`
impl Sub<Z0> for Z0 {
    type Output = Z0;
    fn sub(self, _: Z0) -> Self::Output { unreachable!() }
}

/// `Z0 - P = N`
impl<U: Unsigned + NonZero> Sub<PInt<U>> for Z0 {
    type Output = NInt<U>;
    fn sub(self, _: PInt<U>) -> Self::Output { unreachable!() }
}

/// `Z0 - N = P`
impl<U: Unsigned + NonZero> Sub<NInt<U>> for Z0 {
    type Output = PInt<U>;
    fn sub(self, _: NInt<U>) -> Self::Output { unreachable!() }
}

/// `PInt - Z0 = PInt`
impl<U: Unsigned + NonZero> Sub<Z0> for PInt<U> {
    type Output = PInt<U>;
    fn sub(self, _: Z0) -> Self::Output { unreachable!() }
}

/// `NInt - Z0 = NInt`
impl<U: Unsigned + NonZero> Sub<Z0> for NInt<U> {
    type Output = NInt<U>;
    fn sub(self, _: Z0) -> Self::Output { unreachable!() }
}

/// `P(Ul) - N(Ur) = P(Ul + Ur)`
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Sub<NInt<Ur>> for PInt<Ul>
    where Ul: Add<Ur>,
          <Ul as Add<Ur>>::Output: Unsigned + NonZero
{
    type Output = PInt<<Ul as Add<Ur>>::Output>;
    fn sub(self, _: NInt<Ur>) -> Self::Output { unreachable!() }
}

/// `N(Ul) - P(Ur) = N(Ul + Ur)`
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Sub<PInt<Ur>> for NInt<Ul>
    where Ul: Add<Ur>,
          <Ul as Add<Ur>>::Output: Unsigned + NonZero
{
    type Output = NInt<<Ul as Add<Ur>>::Output>;
    fn sub(self, _: PInt<Ur>) -> Self::Output { unreachable!() }
}

/// `P(Ul) - P(Ur)`: We resolve this with our `PrivateAdd`
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Sub<PInt<Ur>> for PInt<Ul>
    where Ul: Cmp<Ur> + PrivateIntegerAdd<<Ul as Cmp<Ur>>::Output, Ur>
{
    type Output = <Ul as PrivateIntegerAdd<
        <Ul as Cmp<Ur>>::Output, Ur
        >>::Output;
    fn sub(self, _: PInt<Ur>) -> Self::Output { unreachable!() }
}

/// `N(Ul) - N(Ur)`: We resolve this with our `PrivateAdd`
// We just do the same thing as above, swapping Lhs and Rhs
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Sub<NInt<Ur>> for NInt<Ul>
    where Ur: Cmp<Ul> + PrivateIntegerAdd<<Ur as Cmp<Ul>>::Output, Ul>
{
    type Output = <Ur as PrivateIntegerAdd<
        <Ur as Cmp<Ul>>::Output, Ul
        >>::Output;
    fn sub(self, _: NInt<Ur>) -> Self::Output { unreachable!() }
}

// ---------------------------------------------------------------------------------------
// Mul

/// `Z0 * I = Z0`
impl<I: Integer> Mul<I> for Z0 {
    type Output = Z0;
    fn mul(self, _: I) -> Self::Output { unreachable!() }
}

/// `P * Z0 = Z0`
impl<U: Unsigned + NonZero> Mul<Z0> for PInt<U> {
    type Output = Z0;
    fn mul(self, _: Z0) -> Self::Output { unreachable!() }
}

/// `N * Z0 = Z0`
impl<U: Unsigned + NonZero> Mul<Z0> for NInt<U> {
    type Output = Z0;
    fn mul(self, _: Z0) -> Self::Output { unreachable!() }
}

/// P(Ul) * P(Ur) = P(Ul * Ur)
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Mul<PInt<Ur>> for PInt<Ul>
    where Ul: Mul<Ur>,
          <Ul as Mul<Ur>>::Output: Unsigned + NonZero
{
    type Output = PInt<<Ul as Mul<Ur>>::Output>;
    fn mul(self, _: PInt<Ur>) -> Self::Output { unreachable!() }
}

/// N(Ul) * N(Ur) = P(Ul * Ur)
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Mul<NInt<Ur>> for NInt<Ul>
    where Ul: Mul<Ur>,
          <Ul as Mul<Ur>>::Output: Unsigned + NonZero
{
    type Output = PInt<<Ul as Mul<Ur>>::Output>;
    fn mul(self, _: NInt<Ur>) -> Self::Output { unreachable!() }
}

/// P(Ul) * N(Ur) = N(Ul * Ur)
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Mul<NInt<Ur>> for PInt<Ul>
    where Ul: Mul<Ur>,
          <Ul as Mul<Ur>>::Output: Unsigned + NonZero
{
    type Output = NInt<<Ul as Mul<Ur>>::Output>;
    fn mul(self, _: NInt<Ur>) -> Self::Output { unreachable!() }
}

/// N(Ul) * P(Ur) = N(Ul * Ur)
impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Mul<PInt<Ur>> for NInt<Ul>
    where Ul: Mul<Ur>,
          <Ul as Mul<Ur>>::Output: Unsigned + NonZero
{
    type Output = NInt<<Ul as Mul<Ur>>::Output>;
    fn mul(self, _: PInt<Ur>) -> Self::Output { unreachable!() }
}

// ---------------------------------------------------------------------------------------
// Div

/// `Z0 / I = Z0` where `I != 0`
impl<I: Integer + NonZero> Div<I> for Z0 {
    type Output = Z0;
    fn div(self, _: I) -> Self::Output { unreachable!() }
}

macro_rules! impl_int_div {
    ($A:ident, $B:ident, $R:ident) => (
        /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
        impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
            where Ul: Cmp<Ur>,
                  $A<Ul>: PrivateDivFirstStep<<Ul as Cmp<Ur>>::Output, $B<Ur>>
        {
            type Output = <$A<Ul> as PrivateDivFirstStep<
                <Ul as Cmp<Ur>>::Output,
                $B<Ur>>>::Output;
            fn div(self, _: $B<Ur>) -> Self::Output { unreachable!() }
        }
        impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateDivFirstStep<Less, $B<Ur>> for $A<Ul> {
            type Output = Z0;
        }
        impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateDivFirstStep<Equal, $B<Ur>> for $A<Ul> {
            type Output = $R<U1>;
        }
        impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateDivFirstStep<Greater, $B<Ur>> for $A<Ul>
            where Ul: Div<Ur>,
                  <Ul as Div<Ur>>::Output: Unsigned + NonZero
        {
            type Output = $R<<Ul as Div<Ur>>::Output>;
        }
        );
}

impl_int_div!(PInt, PInt, PInt);
impl_int_div!(PInt, NInt, NInt);
impl_int_div!(NInt, PInt, NInt);
impl_int_div!(NInt, NInt, PInt);

// ---------------------------------------------------------------------------------------
// Cmp

/// 0 == 0
impl Cmp<Z0> for Z0 {
    type Output = Equal;
}

/// 0 > -X
impl<U: Unsigned + NonZero> Cmp<NInt<U>> for Z0 {
    type Output = Greater;
}

/// 0 < X
impl<U: Unsigned + NonZero> Cmp<PInt<U>> for Z0 {
    type Output = Less;
}

/// X > 0
impl<U: Unsigned + NonZero> Cmp<Z0> for PInt<U> {
    type Output = Greater;
}

/// -X < 0
impl<U: Unsigned + NonZero> Cmp<Z0> for NInt<U> {
    type Output = Less;
}

/// -X < Y
impl<P: Unsigned + NonZero, N: Unsigned + NonZero> Cmp<PInt<P>> for NInt<N> {
    type Output = Less;
}

/// X > - Y
impl<P: Unsigned + NonZero, N: Unsigned + NonZero> Cmp<NInt<N>> for PInt<P> {
    type Output = Greater;
}

/// X <==> Y
impl<Pl: Cmp<Pr> + Unsigned + NonZero, Pr: Unsigned + NonZero> Cmp<PInt<Pr>> for PInt<Pl> {
    type Output = <Pl as Cmp<Pr>>::Output;
}

/// -X <==> -Y
impl<Nl: Unsigned + NonZero, Nr: Cmp<Nl> + Unsigned + NonZero> Cmp<NInt<Nr>> for NInt<Nl> {
    type Output = <Nr as Cmp<Nl>>::Output;
}

macro_rules! test_ord {
    ($Lhs:ident > $Rhs:ident) => (
        {
            type Test = <$Lhs as Cmp<$Rhs>>::Output;
            assert_eq!(::std::cmp::Ordering::Greater, <Test as Ord>::to_ordering());
        }
        );
    ($Lhs:ident == $Rhs:ident) => (
        {
            type Test = <$Lhs as Cmp<$Rhs>>::Output;
            assert_eq!(::std::cmp::Ordering::Equal, <Test as Ord>::to_ordering());
        }
        );
    ($Lhs:ident < $Rhs:ident) => (
        {
            type Test = <$Lhs as Cmp<$Rhs>>::Output;
            assert_eq!(::std::cmp::Ordering::Less, <Test as Ord>::to_ordering());
        }
        );
}

// ---------------------------------------------------------------------------------------
// Pow

impl Pow<Z0> for Z0 {
    type Output = P1;
}

impl<U: Unsigned> Pow<PInt<U>> for Z0 {
    type Output = Z0;
}

impl<U: Unsigned> Pow<NInt<U>> for Z0 {
    type Output = Z0;
}

impl<U: Unsigned> Pow<NInt<U>> for P1 {
    type Output = P1;
}

impl<Ul: Unsigned, Ur: Unsigned> Pow<PInt<Ur>> for PInt<Ul> where Ul: Pow<Ur> {
    type Output = PInt<<Ul as Pow<Ur>>::Output>
}

// fixme: use `Rem` to find if `Ur` is even
// impl<Ul: Unsigned, Ur: Unsigned> Pow<PInt<Ur>> for NInt<Ul> where Ul: Pow<Ur> {
//     type Output = PInt<<Ul as Pow<Ur>>::Output>
// }
