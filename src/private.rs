/*!
This module is for things that are conceptually private but that must be made public for
typenum to work correctly.

Unless you are working on typenum itself, **there is need to view anything here**.

Certainly don't implement any of the traits here for anything.


Just look away.


Loooooooooooooooooooooooooooooooooook awaaaaaaaaaaaayyyyyyyyyyyyyyyyyyyyyyyyyyyyy...
 */

use std::marker::PhantomData;

// use ::{Sub};
use ::bit::{Bit, B1, B0};
use ::uint::{Unsigned, UInt, UTerm};

/// Convenience trait. Calls Invert -> TrimTrailingZeros -> Invert
pub trait Trim {
    type Output;
}

/// Gets rid of all zeros until it hits a one.

// ONLY IMPLEMENT FOR INVERTED NUMBERS!
pub trait TrimTrailingZeros {
    type Output;
}

/// Converts between standard numbers and inverted ones that have the most significant
/// digit on the outside.
pub trait Invert {
    type Output;
}
/// Doubly private! Called by invert to make the magic happen once its done the first step.
/// The Rhs is what we've got so far.
pub trait PrivateInvert<Rhs> {
    type Output;
}

/// Terminating character for `InvertedUInt`s
pub struct InvertedUTerm;

/// Inverted UInt (has most significant digit on the outside)
pub struct InvertedUInt<IU: InvertedUnsigned, B: Bit> {
    _marker: PhantomData<(IU, B)>
}

/// Does the real anding for `UInt`s; `And` just calls this and then `Trim`.
pub trait PrivateAnd<Rhs = Self> {
    type Output;
}

/// Does the real xoring for `UInt`s; `Xor` just calls this and then `Trim`.
pub trait PrivateXor<Rhs = Self> {
    type Output;
}

/// Does the real subtraction for `UInt`s; `Sub` just calls this and then `Trim`.
pub trait PrivateSub<Rhs = Self> {
    type Output;
}
/// Inverted unsigned numbers
pub trait InvertedUnsigned {
    fn to_int() -> u64;
}

impl InvertedUnsigned for InvertedUTerm {
    fn to_int() -> u64 { 0 }
}

impl<IU: InvertedUnsigned, B: Bit> InvertedUnsigned for InvertedUInt<IU, B> {
    fn to_int() -> u64 {
        B::to_int() as u64 + 2*(IU::to_int())
    }
}

impl Invert for UTerm {
    type Output = InvertedUTerm;
}

impl<U: Unsigned, B: Bit> Invert for UInt<U, B>
    where U: PrivateInvert<InvertedUInt<InvertedUTerm, B>>
{
    type Output = <U as PrivateInvert<InvertedUInt<InvertedUTerm, B>>>::Output;
}


impl<IU: InvertedUnsigned> PrivateInvert<IU> for UTerm {
    type Output = IU;
}

impl<IU: InvertedUnsigned, U: Unsigned, B: Bit> PrivateInvert<IU> for UInt<U, B>
    where U: PrivateInvert<InvertedUInt<IU, B>>
{
    type Output = <U as PrivateInvert<InvertedUInt<IU, B>>>::Output;
}

#[test]
fn test_inversion() {
    type Test4 = <::uint::U4 as Invert>::Output;
    type Test5 = <::uint::U5 as Invert>::Output;
    type Test12 = <::uint::U12 as Invert>::Output;
    type Test16 = <::uint::U16 as Invert>::Output;

    assert_eq!(1, <Test4 as InvertedUnsigned>::to_int());
    assert_eq!(5, <Test5 as InvertedUnsigned>::to_int());
    assert_eq!(3, <Test12 as InvertedUnsigned>::to_int());
    assert_eq!(1, <Test16 as InvertedUnsigned>::to_int());
}

impl Invert for InvertedUTerm {
    type Output = UTerm;
}

impl<IU: InvertedUnsigned, B: Bit> Invert for InvertedUInt<IU, B>
    where IU: PrivateInvert<UInt<UTerm, B>>
{
    type Output = <IU as PrivateInvert<UInt<UTerm, B>>>::Output;
}

impl<U: Unsigned> PrivateInvert<U> for InvertedUTerm {
    type Output = U;
}

impl<U: Unsigned, IU: InvertedUnsigned, B: Bit> PrivateInvert<U> for InvertedUInt<IU, B>
    where IU: PrivateInvert<UInt<U, B>>
{
    type Output = <IU as PrivateInvert<UInt<U, B>>>::Output;
}

#[test]
fn test_double_inversion() {
    type Test4 = <<::uint::U4 as Invert>::Output as Invert>::Output;
    type Test5 = <<::uint::U5 as Invert>::Output as Invert>::Output;
    type Test12 = <<::uint::U12 as Invert>::Output as Invert>::Output;
    type Test16 = <<::uint::U16 as Invert>::Output as Invert>::Output;

    assert_eq!(4, <Test4 as Unsigned>::to_int());
    assert_eq!(5, <Test5 as Unsigned>::to_int());
    assert_eq!(12, <Test12 as Unsigned>::to_int());
    assert_eq!(16, <Test16 as Unsigned>::to_int());
}

impl TrimTrailingZeros for InvertedUTerm {
    type Output = InvertedUTerm;
}

impl<IU: InvertedUnsigned> TrimTrailingZeros for InvertedUInt<IU, B1> {
    type Output = Self;
}

impl<IU: InvertedUnsigned> TrimTrailingZeros for InvertedUInt<IU, B0>
    where IU: TrimTrailingZeros
{
    type Output = <IU as TrimTrailingZeros>::Output;
}

impl<U: Unsigned> Trim for U
    where U: Invert,
          <U as Invert>::Output: TrimTrailingZeros,
          <<U as Invert>::Output as TrimTrailingZeros>::Output: Invert
{
    type Output = <<<U as Invert>::Output as TrimTrailingZeros>::Output as Invert>::Output;
}

// Note: Trimming is tested when we do subtraction.
