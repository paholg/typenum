/*!

Type-level unsigned integers.


*Type operators** implemented:

From core::ops: `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`, `Add`, `Sub`, `Mul`, `Div`, and `Rem`.
From typenum: `Same`, `Cmp`, and `Pow`.

Rather than directly using the structs defined in this module, it is recommended that
you import and use the relevant aliases from the [consts](../consts/index.html) module.

# Example
```rust
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul, Div, Rem};
use typenum::{Unsigned, U1, U2, U3, U4};

assert_eq!(<U3 as BitAnd<U2>>::Output::to_u32(), 2);
assert_eq!(<U3 as BitOr<U4>>::Output::to_u32(), 7);
assert_eq!(<U3 as BitXor<U2>>::Output::to_u32(), 1);
assert_eq!(<U3 as Shl<U1>>::Output::to_u32(), 6);
assert_eq!(<U3 as Shr<U1>>::Output::to_u32(), 1);
assert_eq!(<U3 as Add<U2>>::Output::to_u32(), 5);
assert_eq!(<U3 as Sub<U2>>::Output::to_u32(), 1);
assert_eq!(<U3 as Mul<U2>>::Output::to_u32(), 6);
assert_eq!(<U3 as Div<U2>>::Output::to_u32(), 1);
assert_eq!(<U3 as Rem<U2>>::Output::to_u32(), 1);
```
*/

use core::marker::PhantomData;

use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul, Div, Rem};
use {NonZero, Ord, Greater, Equal, Less, Pow, Cmp};
use bit::{Bit, B0, B1};

use private::{Trim, SizeOf, PrivateAnd, PrivateXor, PrivateSub, PrivateCmp,
              PrivateSizeOf, ShiftDiff, PrivateDiv, PrivateDivFirstStep, PrivatePow,
              BitDiff};

use private::{TrimOut, SizeOfOut, PrivateAndOut, PrivateXorOut, PrivateSubOut,
              PrivateCmpOut, PrivateSizeOfOut, PrivatePowOut, BitDiffOut, ShiftDiffOut,
              PrivateDivFirstStepQuot, PrivateDivFirstStepRem, PrivateDivQuot,
              PrivateDivRem};

use consts::{U0, U1};
use {Or, Shleft, Shright, Sum, Prod, Diff, Add1, Sub1, Square, Compare};

pub use marker_traits::Unsigned;

/**
The terminating type for `UInt`; it always comes after the most significant bit. `UTerm`
 by itself represents zero, which is aliased to `U0`.
 */
pub enum UTerm {}
impl_derivable!{UTerm}

impl Unsigned for UTerm {
    #[inline]
    fn to_u8() -> u8 {
        0
    }
    #[inline]
    fn to_u16() -> u16 {
        0
    }
    #[inline]
    fn to_u32() -> u32 {
        0
    }
    #[inline]
    fn to_u64() -> u64 {
        0
    }
    #[inline]
    fn to_usize() -> usize {
        0
    }

    #[inline]
    fn to_i8() -> i8 {
        0
    }
    #[inline]
    fn to_i16() -> i16 {
        0
    }
    #[inline]
    fn to_i32() -> i32 {
        0
    }
    #[inline]
    fn to_i64() -> i64 {
        0
    }
    #[inline]
    fn to_isize() -> isize {
        0
    }
}

/**
`UInt` is defined recursively, where `B` is the least significant bit and `U` is the rest
of the number. Conceptually, `U` should be bound by the trait `Unsigned` and `B` should
be bound by the trait `Bit`, but enforcing these bounds causes linear instead of
logrithmic scaling in some places, so they are left off for now. They may be enforced in
future.

In order to keep numbers unique, leading zeros are not allowed, so `UInt<UTerm, B0>` is
forbidden.

# Example
```rust
use typenum::{B0, B1, UInt, UTerm, Unsigned};

type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
```
 */
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct UInt<U, B> {
    _marker: PhantomData<(U, B)>,
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {
    #[inline]
    fn to_u8() -> u8 {
        B::to_u8() | U::to_u8() << 1
    }
    #[inline]
    fn to_u16() -> u16 {
        B::to_u8() as u16 | U::to_u16() << 1
    }
    #[inline]
    fn to_u32() -> u32 {
        B::to_u8() as u32 | U::to_u32() << 1
    }
    #[inline]
    fn to_u64() -> u64 {
        B::to_u8() as u64 | U::to_u64() << 1
    }
    #[inline]
    fn to_usize() -> usize {
        B::to_u8() as usize | U::to_usize() << 1
    }

    #[inline]
    fn to_i8() -> i8 {
        B::to_u8() as i8 | U::to_i8() << 1
    }
    #[inline]
    fn to_i16() -> i16 {
        B::to_u8() as i16 | U::to_i16() << 1
    }
    #[inline]
    fn to_i32() -> i32 {
        B::to_u8() as i32 | U::to_i32() << 1
    }
    #[inline]
    fn to_i64() -> i64 {
        B::to_u8() as i64 | U::to_i64() << 1
    }
    #[inline]
    fn to_isize() -> isize {
        B::to_u8() as isize | U::to_isize() << 1
    }
}

impl<U: Unsigned, B: Bit> NonZero for UInt<U, B> {}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_uint_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as ::Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_u64(), <Test as Unsigned>::to_u64());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as ::Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_u64(), <Test as Unsigned>::to_u64());
        }
        );
}

// ---------------------------------------------------------------------------------------
// Getting size of unsigned integers

/// Size of `UTerm` by itself is 0
impl SizeOf for UTerm {
    type Output = U0;
}

/// Size of a `UInt`
impl<U: Unsigned, B: Bit> SizeOf for UInt<U, B> where UInt<U, B>: PrivateSizeOf
{
    type Output = PrivateSizeOfOut<UInt<U, B>>;
}

/// Size of `UTerm` inside a number is 0
impl PrivateSizeOf for UTerm {
    type Output = U0;
}

/// Size of bit is 1
impl<U: Unsigned, B: Bit> PrivateSizeOf for UInt<U, B>
    where U: PrivateSizeOf,
          PrivateSizeOfOut<U>: Add<B1>
{
    type Output = Add1<PrivateSizeOfOut<U>>;
}

// ---------------------------------------------------------------------------------------
// Adding bits to unsigned integers

/// `UTerm + B0 = UTerm`
impl Add<B0> for UTerm {
    type Output = UTerm;
    fn add(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// `U + B0 = U`
impl<U: Unsigned, B: Bit> Add<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn add(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// `UTerm + B1 = UInt<UTerm, B1>`
impl Add<B1> for UTerm {
    type Output = UInt<UTerm, B1>;
    fn add(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<U, B0> + B1 = UInt<U + B1>`
impl<U: Unsigned> Add<B1> for UInt<U, B0> {
    type Output = UInt<U, B1>;
    fn add(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<U, B1> + B1 = UInt<U + B1, B0>`
impl<U: Unsigned> Add<B1> for UInt<U, B1>
    where U: Add<B1>,
          Sum<U, B1>: Unsigned
{
    type Output = UInt<Add1<U>, B0>;
    fn add(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Adding unsigned integers

/// `UTerm + U = U`
impl<U: Unsigned> Add<U> for UTerm {
    type Output = U;
    fn add(self, _: U) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<U, B> + UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Add<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn add(self, _: UTerm) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B0> + UInt<Ur, B0> = UInt<Ul + Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: Add<Ur>
{
    type Output = UInt<Sum<Ul, Ur>, B0>;
    fn add(self, _: UInt<Ur, B0>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B0> + UInt<Ur, B1> = UInt<Ul + Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B1>> for UInt<Ul, B0> where Ul: Add<Ur>
{
    type Output = UInt<Sum<Ul, Ur>, B1>;
    fn add(self, _: UInt<Ur, B1>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B1> + UInt<Ur, B0> = UInt<Ul + Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: Add<Ur>
{
    type Output = UInt<Sum<Ul, Ur>, B1>;
    fn add(self, _: UInt<Ur, B0>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B1> + UInt<Ur, B1> = UInt<(Ul + Ur) + B1, B0>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B1>> for UInt<Ul, B1>
    where Ul: Add<Ur>,
          Sum<Ul, Ur>: Add<B1>
{
    type Output = UInt<Add1<Sum<Ul, Ur>>, B0>;
    fn add(self, _: UInt<Ur, B1>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Subtracting bits from unsigned integers

/// `UTerm - B0 = Term`
impl Sub<B0> for UTerm {
    type Output = UTerm;
    fn sub(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// `UInt - B0 = UInt`
impl<U: Unsigned, B: Bit> Sub<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn sub(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<U, B1> - B1 = UInt<U, B0>`
impl<U: Unsigned, B: Bit> Sub<B1> for UInt<UInt<U, B>, B1> {
    type Output = UInt<UInt<U, B>, B0>;
    fn sub(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<UTerm, B1> - B1 = UTerm`
impl Sub<B1> for UInt<UTerm, B1> {
    type Output = UTerm;
    fn sub(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<U, B0> - B1 = UInt<U - B1, B1>`
impl<U: Unsigned> Sub<B1> for UInt<U, B0>
    where U: Sub<B1>,
          Sub1<U>: Unsigned
{
    type Output = UInt<Sub1<U>, B1>;
    fn sub(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Subtracting unsigned integers

/// `UTerm - UTerm = UTerm`
impl Sub<UTerm> for UTerm {
    type Output = UTerm;
    fn sub(self, _: UTerm) -> Self::Output {
        unreachable!()
    }
}

/// Subtracting unsigned integers. We just do our `PrivateSub` and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> Sub<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateSub<Ur>,
          PrivateSubOut<UInt<Ul, Bl>, Ur>: Trim
{
    type Output = TrimOut<PrivateSubOut<UInt<Ul, Bl>, Ur>>;
    fn sub(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

/// `U - UTerm = U`
impl<U: Unsigned> PrivateSub<UTerm> for U {
    type Output = U;
}

/// `UInt<Ul, B0> - UInt<Ur, B0> = UInt<Ul - Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: PrivateSub<Ur>
{
    type Output = UInt<PrivateSubOut<Ul, Ur>, B0>;
}

/// `UInt<Ul, B0> - UInt<Ur, B1> = UInt<(Ul - Ur) - B1, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B1>> for UInt<Ul, B0>
    where Ul: PrivateSub<Ur>,
          PrivateSubOut<Ul, Ur>: Sub<B1>
{
    type Output = UInt<Sub1<PrivateSubOut<Ul, Ur>>, B1>;
}

/// `UInt<Ul, B1> - UInt<Ur, B0> = UInt<Ul - Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: PrivateSub<Ur>
{
    type Output = UInt<PrivateSubOut<Ul, Ur>, B1>;
}

/// `UInt<Ul, B1> - UInt<Ur, B1> = UInt<Ul - Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B1>> for UInt<Ul, B1> where Ul: PrivateSub<Ur>
{
    type Output = UInt<PrivateSubOut<Ul, Ur>, B0>;
}

// ---------------------------------------------------------------------------------------
// And unsigned integers

/// 0 & X = 0
impl<Ur: Unsigned> BitAnd<Ur> for UTerm {
    type Output = UTerm;
    fn bitand(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

/// Anding unsigned integers.
/// We use our `PrivateAnd` operator and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> BitAnd<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateAnd<Ur>,
          PrivateAndOut<UInt<Ul, Bl>, Ur>: Trim
{
    type Output = TrimOut<PrivateAndOut<UInt<Ul, Bl>, Ur>>;
    fn bitand(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

/// `UTerm & X = UTerm`
impl<U: Unsigned> PrivateAnd<U> for UTerm {
    type Output = UTerm;
}

/// `X & UTerm = UTerm`
impl<B: Bit, U: Unsigned> PrivateAnd<UTerm> for UInt<U, B> {
    type Output = UTerm;
}

/// `UInt<Ul, B0> & UInt<Ur, B0> = UInt<Ul & Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: PrivateAnd<Ur>
{
    type Output = UInt<PrivateAndOut<Ul, Ur>, B0>;
}

/// `UInt<Ul, B0> & UInt<Ur, B1> = UInt<Ul & Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B1>> for UInt<Ul, B0> where Ul: PrivateAnd<Ur>
{
    type Output = UInt<PrivateAndOut<Ul, Ur>, B0>;
}

/// `UInt<Ul, B1> & UInt<Ur, B0> = UInt<Ul & Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: PrivateAnd<Ur>
{
    type Output = UInt<PrivateAndOut<Ul, Ur>, B0>;
}

/// `UInt<Ul, B1> & UInt<Ur, B1> = UInt<Ul & Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B1>> for UInt<Ul, B1> where Ul: PrivateAnd<Ur>
{
    type Output = UInt<PrivateAndOut<Ul, Ur>, B1>;
}

// ---------------------------------------------------------------------------------------
// Or unsigned integers

/// `UTerm | X = X`
impl<U: Unsigned> BitOr<U> for UTerm {
    type Output = U;
    fn bitor(self, _: U) -> Self::Output {
        unreachable!()
    }
}

///  `X | UTerm = X`
impl<B: Bit, U: Unsigned> BitOr<UTerm> for UInt<U, B> {
    type Output = Self;
    fn bitor(self, _: UTerm) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B0> | UInt<Ur, B0> = UInt<Ul | Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: BitOr<Ur>
{
    type Output = UInt<<Ul as BitOr<Ur>>::Output, B0>;
    fn bitor(self, _: UInt<Ur, B0>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B0> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B1>> for UInt<Ul, B0> where Ul: BitOr<Ur>
{
    type Output = UInt<Or<Ul, Ur>, B1>;
    fn bitor(self, _: UInt<Ur, B1>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B1> | UInt<Ur, B0> = UInt<Ul | Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: BitOr<Ur>
{
    type Output = UInt<Or<Ul, Ur>, B1>;
    fn bitor(self, _: UInt<Ur, B0>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B1> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B1>> for UInt<Ul, B1> where Ul: BitOr<Ur>
{
    type Output = UInt<Or<Ul, Ur>, B1>;
    fn bitor(self, _: UInt<Ur, B1>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Xor unsigned integers

/// 0 ^ X = X
impl<Ur: Unsigned> BitXor<Ur> for UTerm {
    type Output = Ur;
    fn bitxor(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

/// Xoring unsigned integers.
/// We use our `PrivateXor` operator and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> BitXor<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateXor<Ur>,
          PrivateXorOut<UInt<Ul, Bl>, Ur>: Trim
{
    type Output = TrimOut<PrivateXorOut<UInt<Ul, Bl>, Ur>>;
    fn bitxor(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

/// `UTerm ^ X = X`
impl<U: Unsigned> PrivateXor<U> for UTerm {
    type Output = U;
}

/// `X ^ UTerm = X`
impl<B: Bit, U: Unsigned> PrivateXor<UTerm> for UInt<U, B> {
    type Output = Self;
}

/// `UInt<Ul, B0> ^ UInt<Ur, B0> = UInt<Ul ^ Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: PrivateXor<Ur>
{
    type Output = UInt<PrivateXorOut<Ul, Ur>, B0>;
}

/// `UInt<Ul, B0> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B1>> for UInt<Ul, B0> where Ul: PrivateXor<Ur>
{
    type Output = UInt<PrivateXorOut<Ul, Ur>, B1>;
}

/// `UInt<Ul, B1> ^ UInt<Ur, B0> = UInt<Ul ^ Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: PrivateXor<Ur>
{
    type Output = UInt<PrivateXorOut<Ul, Ur>, B1>;
}

/// `UInt<Ul, B1> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B1>> for UInt<Ul, B1> where Ul: PrivateXor<Ur>
{
    type Output = UInt<PrivateXorOut<Ul, Ur>, B0>;
}

// ---------------------------------------------------------------------------------------
// Shl unsigned integers

/// Shifting left `UTerm` by an unsigned integer: `UTerm << U = UTerm`
impl<U: Unsigned> Shl<U> for UTerm {
    type Output = UTerm;
    fn shl(self, _: U) -> Self::Output {
        unreachable!()
    }
}

/// Shifting left `UInt` by `UTerm`: `UInt<U, B> << UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Shl<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shl(self, _: UTerm) -> Self::Output {
        unreachable!()
    }
}

/// Shifting left any unsigned by a zero bit: `U << B0 = U`
impl<U: Unsigned, B: Bit> Shl<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shl(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// Shifting UTerm by a 0 bit: `UTerm << B0 = UTerm`
impl Shl<B0> for UTerm {
    type Output = UTerm;
    fn shl(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// Shifting UTerm by a 1 bit: `UTerm << B1 = UTerm`
impl Shl<B1> for UTerm {
    type Output = UTerm;
    fn shl(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// Shifting left a `UInt` by a one bit: `UInt<U, B> << B1 = UInt<UInt<U, B>, B0>`
impl<U: Unsigned, B: Bit> Shl<B1> for UInt<U, B> {
    type Output = UInt<UInt<U, B>, B0>;
    fn shl(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// Shifting left `UInt` by `UInt`: `X << Y` = `UInt(X, B0) << (Y - 1)`
impl<U: Unsigned, B: Bit, Ur: Unsigned, Br: Bit> Shl<UInt<Ur, Br>> for UInt<U, B>
    where UInt<Ur, Br>: Sub<B1>,
          UInt<UInt<U, B>, B0>: Shl<Sub1<UInt<Ur, Br>>>
{
    type Output = Shleft<UInt<UInt<U, B>, B0>, Sub1<UInt<Ur, Br>>>;
    fn shl(self, _: UInt<Ur, Br>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Shr unsigned integers

/// Shifting right a `UTerm` by an unsigned integer: `UTerm >> U = UTerm`
impl<U: Unsigned> Shr<U> for UTerm {
    type Output = UTerm;
    fn shr(self, _: U) -> Self::Output {
        unreachable!()
    }
}

/// Shifting right `UInt` by `UTerm`: `UInt<U, B> >> UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Shr<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shr(self, _: UTerm) -> Self::Output {
        unreachable!()
    }
}

/// Shifting right UTerm by a 0 bit: `UTerm >> B0 = UTerm`
impl Shr<B0> for UTerm {
    type Output = UTerm;
    fn shr(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// Shifting right UTerm by a 1 bit: `UTerm >> B1 = UTerm`
impl Shr<B1> for UTerm {
    type Output = UTerm;
    fn shr(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// Shifting right any unsigned by a zero bit: `U >> B0 = U`
impl<U: Unsigned, B: Bit> Shr<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shr(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// Shifting right a `UInt` by a 1 bit: `UInt<U, B> >> B1 = U`
impl<U: Unsigned, B: Bit> Shr<B1> for UInt<U, B> {
    type Output = U;
    fn shr(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// Shifting right `UInt` by `UInt`: `UInt(U, B) >> Y` = `U >> (Y - 1)`
impl<U: Unsigned, B: Bit, Ur: Unsigned, Br: Bit> Shr<UInt<Ur, Br>> for UInt<U, B>
    where UInt<Ur, Br>: Sub<B1>,
          U: Shr<Sub1<UInt<Ur, Br>>>
{
    type Output = Shright<U, Sub1<UInt<Ur, Br>>>;
    fn shr(self, _: UInt<Ur, Br>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Multiply unsigned integers

/// `UInt * B0 = UTerm`
impl<U: Unsigned, B: Bit> Mul<B0> for UInt<U, B> {
    type Output = UTerm;
    fn mul(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// `UTerm * B0 = UTerm`
impl Mul<B0> for UTerm {
    type Output = UTerm;
    fn mul(self, _: B0) -> Self::Output {
        unreachable!()
    }
}

/// `UTerm * B1 = UTerm`
impl Mul<B1> for UTerm {
    type Output = UTerm;
    fn mul(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// `UInt * B1 = UInt`
impl<U: Unsigned, B: Bit> Mul<B1> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn mul(self, _: B1) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<U, B> * UTerm = UTerm`
impl<U: Unsigned, B: Bit> Mul<UTerm> for UInt<U, B> {
    type Output = UTerm;
    fn mul(self, _: UTerm) -> Self::Output {
        unreachable!()
    }
}

/// `UTerm * U = UTerm`
impl<U: Unsigned> Mul<U> for UTerm {
    type Output = UTerm;
    fn mul(self, _: U) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B0> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0>`
impl<Ul: Unsigned, B: Bit, Ur: Unsigned> Mul<UInt<Ur, B>> for UInt<Ul, B0>
    where Ul: Mul<UInt<Ur, B>>
{
    type Output = UInt<Prod<Ul, UInt<Ur, B>>, B0>;
    fn mul(self, _: UInt<Ur, B>) -> Self::Output {
        unreachable!()
    }
}

/// `UInt<Ul, B1> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0> + UInt<Ur, B>`
impl<Ul: Unsigned, B: Bit, Ur: Unsigned> Mul<UInt<Ur, B>> for UInt<Ul, B1>
    where Ul: Mul<UInt<Ur, B>>,
          UInt<Prod<Ul, UInt<Ur, B>>, B0>: Add<UInt<Ur, B>>
{
    type Output = Sum<UInt<Prod<Ul, UInt<Ur, B>>, B0>, UInt<Ur, B>>;
    fn mul(self, _: UInt<Ur, B>) -> Self::Output {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------------------
// Compare unsigned integers

/// Zero == Zero
impl Cmp<UTerm> for UTerm {
    type Output = Equal;
}

/// Nonzero > Zero
impl<U: Unsigned, B: Bit> Cmp<UTerm> for UInt<U, B> {
    type Output = Greater;
}

/// Zero < Nonzero
impl<U: Unsigned, B: Bit> Cmp<UInt<U, B>> for UTerm {
    type Output = Less;
}

/// UInt<Ul, B0> cmp with UInt<Ur, B0>: SoFar is Equal
impl<Ul: Unsigned, Ur: Unsigned> Cmp<UInt<Ur, B0>> for UInt<Ul, B0>
    where Ul: PrivateCmp<Ur, Equal>
{
    type Output = PrivateCmpOut<Ul, Ur, Equal>;
}

/// UInt<Ul, B1> cmp with UInt<Ur, B1>: SoFar is Equal
impl<Ul: Unsigned, Ur: Unsigned> Cmp<UInt<Ur, B1>> for UInt<Ul, B1>
    where Ul: PrivateCmp<Ur, Equal>
{
    type Output = PrivateCmpOut<Ul, Ur, Equal>;
}

/// UInt<Ul, B0> cmp with UInt<Ur, B1>: SoFar is Less
impl<Ul: Unsigned, Ur: Unsigned> Cmp<UInt<Ur, B1>> for UInt<Ul, B0>
    where Ul: PrivateCmp<Ur, Less>
{
    type Output = PrivateCmpOut<Ul, Ur, Less>;
}

/// UInt<Ul, B1> cmp with UInt<Ur, B0>: SoFar is Greater
impl<Ul: Unsigned, Ur: Unsigned> Cmp<UInt<Ur, B0>> for UInt<Ul, B1>
    where Ul: PrivateCmp<Ur, Greater>
{
    type Output = PrivateCmpOut<Ul, Ur, Greater>;
}

/// Comparing non-terimal bits, with both having bit B0. These are the same, so we propogate `SoFar`.
impl<Ul, Ur, SoFar> PrivateCmp<UInt<Ur, B0>, SoFar> for UInt<Ul, B0>
    where Ul: Unsigned,
          Ur: Unsigned,
          SoFar: Ord,
          Ul: PrivateCmp<Ur, SoFar>
{
    type Output = PrivateCmpOut<Ul, Ur, SoFar>;
}

/// Comparing non-terimal bits, with both having bit B1. These are the same, so we propogate `SoFar`.
impl<Ul, Ur, SoFar> PrivateCmp<UInt<Ur, B1>, SoFar> for UInt<Ul, B1>
    where Ul: Unsigned,
          Ur: Unsigned,
          SoFar: Ord,
          Ul: PrivateCmp<Ur, SoFar>
{
    type Output = PrivateCmpOut<Ul, Ur, SoFar>;
}

/// Comparing non-terimal bits, with Lhs having bit B0 and Rhs having bit B1. `SoFar`, Lhs is `Less`.
impl<Ul, Ur, SoFar> PrivateCmp<UInt<Ur, B1>, SoFar> for UInt<Ul, B0>
    where Ul: Unsigned,
          Ur: Unsigned,
          SoFar: Ord,
          Ul: PrivateCmp<Ur, Less>
{
    type Output = PrivateCmpOut<Ul, Ur, Less>;
}

/// Comparing non-terimal bits, with Lhs having bit B1 and Rhs having bit B0. `SoFar`, Lhs is `Greater`.
impl<Ul, Ur, SoFar> PrivateCmp<UInt<Ur, B0>, SoFar> for UInt<Ul, B1>
    where Ul: Unsigned,
          Ur: Unsigned,
          SoFar: Ord,
          Ul: PrivateCmp<Ur, Greater>
{
    type Output = PrivateCmpOut<Ul, Ur, Greater>;
}

/// Got to the end of just the Lhs. It's `Less`.
impl<U: Unsigned, B: Bit, SoFar: Ord> PrivateCmp<UInt<U, B>, SoFar> for UTerm {
    type Output = Less;
}

/// Got to the end of just the Rhs. Lhs is `Greater`.
impl<U: Unsigned, B: Bit, SoFar: Ord> PrivateCmp<UTerm, SoFar> for UInt<U, B> {
    type Output = Greater;
}

/// Got to the end of both! Return `SoFar`
impl<SoFar: Ord> PrivateCmp<UTerm, SoFar> for UTerm {
    type Output = SoFar;
}

macro_rules! test_ord {
    ($Lhs:ident > $Rhs:ident) => (
        {
            type Test = <$Lhs as Cmp<$Rhs>>::Output;
            assert_eq!(::core::cmp::Ordering::Greater, <Test as Ord>::to_ordering());
        }
        );
    ($Lhs:ident == $Rhs:ident) => (
        {
            type Test = <$Lhs as Cmp<$Rhs>>::Output;
            assert_eq!(::core::cmp::Ordering::Equal, <Test as Ord>::to_ordering());
        }
        );
    ($Lhs:ident < $Rhs:ident) => (
        {
            type Test = <$Lhs as Cmp<$Rhs>>::Output;
            assert_eq!(::core::cmp::Ordering::Less, <Test as Ord>::to_ordering());
        }
        );
}

// ---------------------------------------------------------------------------------------
// Getting difference in number of bits

impl<Ul, Bl, Ur, Br> BitDiff<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Ul: Unsigned,
          Bl: Bit,
          Ur: Unsigned,
          Br: Bit,
          Ul: BitDiff<Ur>
{
    type Output = BitDiffOut<Ul, Ur>;
}

impl<Ul> BitDiff<UTerm> for Ul where Ul: Unsigned + SizeOf
{
    type Output = SizeOfOut<Ul>;
}

// ---------------------------------------------------------------------------------------
// Shifting one number until it's the size of another

impl<Ul: Unsigned, Ur: Unsigned> ShiftDiff<Ur> for Ul
    where Ur: BitDiff<Ul>,
          Ul: Shl<BitDiffOut<Ur, Ul>>
{
    type Output = Shleft<Ul, BitDiffOut<Ur, Ul>>;
}

// ---------------------------------------------------------------------------------------
// Powers of unsigned integers

/// X^N
impl<X: Unsigned, N: Unsigned> Pow<N> for X where X: PrivatePow<U1, N>
{
    type Output = PrivatePowOut<X, U1, N>;
}

impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U0> for X {
    type Output = Y;
}

impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U1> for X where X: Mul<Y>
{
    type Output = Prod<X, Y>;
}

/// N is even
impl<Y: Unsigned, U: Unsigned, B: Bit, X: Unsigned> PrivatePow<Y, UInt<UInt<U, B>, B0>> for X
    where X: Mul,
          Square<X>: PrivatePow<Y, UInt<U, B>>
{
    type Output = PrivatePowOut<Square<X>, Y, UInt<U, B>>;
}

/// N is odd
impl<Y: Unsigned, U: Unsigned, B: Bit, X: Unsigned> PrivatePow<Y, UInt<UInt<U, B>, B1>> for X
    where X: Mul + Mul<Y>,
          Square<X>: PrivatePow<Prod<X, Y>, UInt<U, B>>
{
    type Output = PrivatePowOut<Square<X>, Prod<X, Y>, UInt<U, B>>;
}

// ---------------------------------------------------------------------------------------
// Dividing unsigned integers

// Here is the algorithm we use:
// Div:
//   Call PrivateDivFirstStep with C = Numerator.cmp(Divisor)
// PrivateDivFirstStep:
//   if Numerator < Divisor:
//     return 0
//   if Numerator == Divisor:
//     return 1
//   I = SizeOf(Numerator) - SizeOf(Divisor)
//   Divisor = Divisor << I
//   Call PrivateDiv with C = Numerator.cmp(Divisor), I = I, Q = 0, Remainder = Numerator
// PrivateDiv:
//   if I == 0:
//     if C == Less: # Can't do any more
//       return Q
//     if C == Equal # We are done, no remainder
//       return Q + 1
//     if C == Greater # Same as Equal, but we have a remainder
//       return Q + 1
//   # I > 0
//   if C == Less: # Divisor is too big
//     Call PrivateDiv with Divisor >> 1, I - 1, C = Remainder.cmp(Divisor)
//   if C == Equal: # Sweet, we're done early with no remainder
//     return Q + 2^I
//   if C == Greater: # Do a step and keep going
//     Q += 2^I
//     I -= 1
//     Remainder -= Divisor
//     Divisor = Divisor >> 1
//     C = Remainder.cmp(Divisor)
//     Call PrivateDiv

//  -----------------------------------------
// Div
impl<Ur: Unsigned> Div<Ur> for UTerm {
    type Output = UTerm;
    fn div(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> Div<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: Cmp<Ur>,
          UInt<Ul, Bl>: PrivateDivFirstStep<Compare<UInt<Ul, Bl>, Ur>,
              Ur>
{
    type Output = PrivateDivFirstStepQuot<
            UInt<Ul, Bl>,
            Compare<UInt<Ul, Bl>, Ur>,
            Ur
        >;
    fn div(self, _: Ur) -> Self::Output { unreachable!() }
}

//  -----------------------------------------
// PrivateDivFirstStep

/// Numerator < Denominator: return 0
impl<Divisor: Unsigned, Numerator: Unsigned> PrivateDivFirstStep<Less, Divisor> for Numerator {
    type Quotient = U0;
    type Remainder = Numerator;
}

/// Numerator == Denominator: return 1
impl<Divisor: Unsigned, Numerator: Unsigned> PrivateDivFirstStep<Equal, Divisor> for Numerator {
    type Quotient = U1;
    type Remainder = U0;
}
/// Numerator > Denominator:
/// I = SizeOf(Numerator) - SizeOf(Denominator), Q = 0, Divisor <<= I, C = Numerator.Cmp(Divisor), Remainder = Numerator
/// Call PrivateDiv
impl<Divisor: Unsigned, Numerator: Unsigned> PrivateDivFirstStep<Greater, Divisor> for Numerator
    where Numerator: BitDiff<Divisor> + Cmp<Shleft<Divisor, BitDiffOut<Numerator, Divisor>>>,
          Divisor: Shl<BitDiffOut<Numerator, Divisor>>,
          Numerator: PrivateDiv<
              Compare<Numerator, ShiftDiffOut<Divisor, Numerator>>,
              BitDiffOut<Numerator, Divisor>,
              U0,
              ShiftDiffOut<Divisor, Numerator>
          >,
{
    type Quotient = PrivateDivQuot<
            Numerator,
            Compare<Numerator, ShiftDiffOut<Divisor, Numerator>>,
            BitDiffOut<Numerator, Divisor>, // I
            U0, // Q
            ShiftDiffOut<Divisor, Numerator> // Divisor
        >;
    type Remainder = PrivateDivRem<
            Numerator,
            Compare<Numerator, ShiftDiffOut<Divisor, Numerator>>,
            BitDiffOut<Numerator, Divisor>, // I
            U0, // Q
            ShiftDiffOut<Divisor, Numerator> // Divisor
        >;
}

//  -----------------------------------------
// PrivateDiv with I == 0

/// Remainder < Divisor: return Q
impl<Q, Divisor, Remainder> PrivateDiv<Less, U0, Q, Divisor> for Remainder
    where Q: Unsigned,
          Divisor: Unsigned,
          Remainder: Unsigned
{
    type Quotient = Q;
    type Remainder = Remainder;
}

/// Remainder == Divisor: return Q + 1
impl<Q, Divisor, Remainder> PrivateDiv<Equal, U0, Q, Divisor> for Remainder
    where Q: Unsigned,
          Divisor: Unsigned,
          Remainder: Unsigned,
          Q: Add<B1>
{
    type Quotient = Add1<Q>;
    type Remainder = U0;
}

/// Remainder > Divisor: return Q + 1
impl<Q, Divisor, Remainder> PrivateDiv<Greater, U0, Q, Divisor> for Remainder
    where Q: Unsigned,
          Divisor: Unsigned,
          Remainder: Unsigned,
          Q: Add<B1>,
          Remainder: Sub<Divisor>
{
    type Quotient = Add1<Q>;
    type Remainder = Diff<Remainder, Divisor>;
}

//  -----------------------------------------
// PrivateDiv with I > 0

/// Remainder == Divisor: return Q + 2^I = Q + 1 << I
impl<Ui, Bi, Q, Divisor, Remainder> PrivateDiv<Equal, UInt<Ui, Bi>, Q, Divisor> for Remainder
    where Ui: Unsigned,
          Bi: Bit,
          Q: Unsigned,
          Divisor: Unsigned,
          Remainder: Unsigned,
          U1: Shl<UInt<Ui, Bi>>,
          Q: Add<Shleft<U1, UInt<Ui, Bi>>>
{
    type Quotient = Sum<Q, Shleft<U1, UInt<Ui, Bi>>>;
    type Remainder = U0;
}

/// Remainder < Divisor: Divisor >>= 1, I -= 1, C = Remainder.cmp(Divisor)
/// Call PrivateDiv
impl<Ui, Bi, Q, Divisor, Remainder> PrivateDiv<Less, UInt<Ui, Bi>, Q, Divisor> for Remainder
    where Ui: Unsigned, Bi: Bit, Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          Divisor: Shr<B1>,
          Remainder: Cmp<Shright<Divisor, B1>>,
          UInt<Ui, Bi>: Sub<B1>,
          Remainder: PrivateDiv<
              Compare<Remainder, Shright<Divisor, B1>>,
              Sub1<UInt<Ui, Bi>>,
              Q,
              Shright<Divisor, B1>
          >
{
    type Quotient = PrivateDivQuot<
            Remainder,
            Compare<Remainder, Shright<Divisor, B1>>, // Remainder.cmp(New Divisor)
            Sub1<UInt<Ui, Bi>>,
            Q,
            Shright<Divisor, B1>
        >;

    type Remainder = PrivateDivRem<
            Remainder,
            Compare<Remainder, Shright<Divisor, B1>>, // Remainder.cmp(New Divisor)
            Sub1<UInt<Ui, Bi>>,
            Q,
            Shright<Divisor, B1>
        >;
}

/// Remainder > Divisor:
/// Q += 2^I, I -= 1, R -= D, D >>= 1, C = (new R).cmp(new D)
/// Call PrivateDiv
impl<Ui, Bi, Q, Divisor, Remainder> PrivateDiv<Greater, UInt<Ui, Bi>, Q, Divisor> for Remainder
    where Ui: Unsigned, Bi: Bit, Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          Divisor: Shr<B1>,
          Remainder: Sub<Divisor>,
          Diff<Remainder, Divisor>: Cmp<Shright<Divisor, B1>>,
          UInt<Ui, Bi>: Sub<B1>,
          U1: Shl<UInt<Ui, Bi>>,
          Q: Add<Shleft<U1, UInt<Ui, Bi>>>,
          Diff<Remainder, Divisor>: PrivateDiv<
                  Compare<Diff<Remainder,Divisor>, Shright<Divisor, B1>>,
                  Sub1<UInt<Ui, Bi>>,
                  Sum<Q, Shleft<U1, UInt<Ui, Bi>>>,
                  Shright<Divisor, B1>
              >
{
    type Quotient = PrivateDivQuot<
            Diff<Remainder, Divisor>,
            Compare<Diff<Remainder,Divisor>, Shright<Divisor, B1>>,
            Sub1<UInt<Ui, Bi>>,
            Sum<Q, Shleft<U1, UInt<Ui, Bi>>>,
            Shright<Divisor, B1>
        >;

    type Remainder = PrivateDivRem<
            Diff<Remainder, Divisor>,
            Compare<Diff<Remainder,Divisor>, Shright<Divisor, B1>>,
            Sub1<UInt<Ui, Bi>>,
            Sum<Q, Shleft<U1, UInt<Ui, Bi>>>,
            Shright<Divisor, B1>
        >;
}

// ---------------------------------------------------------------------------------------
// Rem

impl<Ur: Unsigned> Rem<Ur> for UTerm {
    type Output = UTerm;
    fn rem(self, _: Ur) -> Self::Output {
        unreachable!()
    }
}

impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> Rem<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: Cmp<Ur>,
          UInt<Ul, Bl>: PrivateDivFirstStep<Compare<UInt<Ul, Bl>, Ur>,
              Ur>
{
    type Output = PrivateDivFirstStepRem<
            UInt<Ul, Bl>,
            Compare<UInt<Ul, Bl>, Ur>,
            Ur
       >;
    fn rem(self, _: Ur) -> Self::Output { unreachable!() }
}
