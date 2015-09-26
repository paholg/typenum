
use std::marker::PhantomData;

use ::{Same, And, Xor, Add, Sub};
use ::bit::{Bit, B0, B1};

/// This trait is implemented for the all things that a `UInt` can take as a parameter,
/// which is just `UInt` and `UTerm` (used to terminate the `UInt`). It should not be
/// implemented for anything outside this crate.
pub trait Unsigned {
    fn to_int() -> u64;
}

/// The terminating type for `UInt`, it always comes after the most significant bit.
pub struct UTerm;

impl Unsigned for UTerm {
    fn to_int() -> u64 { 0 }
}

/// UInt is defined recursevly, where B is the least significant bit and U is the rest
/// of the number. U can be another UInt or UTerm. In order to keep numbers unique, leading
/// zeros are not allowed, so `UInt<UTerm, B0>` should never show up anywhere ever.
pub struct UInt<U: Unsigned, B: Bit> {
    _marker: PhantomData<(U, B)>
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {
    fn to_int() -> u64 {
        B::to_int() as u64 + 2*(U::to_int())
    }
}

impl<U: Unsigned, B: Bit> Same<UInt<U, B>> for UInt<U, B> {
    type Output = UInt<U, B>;
}

pub type U0 = UTerm;
pub type U1 = UInt<UTerm, B1>;
pub type U2 = UInt<UInt<UTerm, B1>, B0>;
pub type U3 = UInt<UInt<UTerm, B1>, B1>;
pub type U4 = UInt<UInt<UInt<UTerm, B1>, B0>, B0>;
pub type U5 = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
pub type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
pub type U7 = UInt<UInt<UInt<UTerm, B1>, B1>, B1>;
pub type U8 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>;
pub type U9 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>;
pub type U10 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>;
pub type U11 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>;
pub type U12 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>;
pub type U13 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>;
pub type U14 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>;
pub type U15 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>;
pub type U16 = <U15 as Add<B1>>::Output;
pub type U17 = <U16 as Add<B1>>::Output;
pub type U18 = <U17 as Add<B1>>::Output;
pub type U19 = <U18 as Add<B1>>::Output;
pub type U20 = <U19 as Add<B1>>::Output;
pub type U21 = <U20 as Add<B1>>::Output;
pub type U22 = <U21 as Add<B1>>::Output;
pub type U23 = <U22 as Add<B1>>::Output;
pub type U24 = <U23 as Add<B1>>::Output;
pub type U25 = <U24 as Add<B1>>::Output;
pub type U26 = <U25 as Add<B1>>::Output;
pub type U27 = <U26 as Add<B1>>::Output;
pub type U28 = <U27 as Add<B1>>::Output;
pub type U29 = <U28 as Add<B1>>::Output;
pub type U30 = <U29 as Add<B1>>::Output;
pub type U31 = <U30 as Add<B1>>::Output;
pub type U32 = <U31 as Add<B1>>::Output;
pub type U64 = <U32 as Add<U32>>::Output;
pub type U128 = <U64 as Add<U64>>::Output;
pub type U256 = <U128 as Add<U128>>::Output;
pub type U512 = <U256 as Add<U256>>::Output;
pub type U1024 = <U512 as Add<U512>>::Output;
pub type U2048 = <U1024 as Add<U1024>>::Output;
pub type U4096 = <U2048 as Add<U2048>>::Output;
pub type U8192 = <U4096 as Add<U4096>>::Output;
pub type U16384 = <U8192 as Add<U8192>>::Output;
pub type U32768 = <U16384 as Add<U16384>>::Output;

#[test]
fn confirm_nums() {
    assert_eq!(0, U0::to_int());
    assert_eq!(1, U1::to_int());
    assert_eq!(2, U2::to_int());
    assert_eq!(3, U3::to_int());
    assert_eq!(4, U4::to_int());
    assert_eq!(5, U5::to_int());
    assert_eq!(6, U6::to_int());
    assert_eq!(7, U7::to_int());
    assert_eq!(8, U8::to_int());
    assert_eq!(9, U9::to_int());
    assert_eq!(10, U10::to_int());
    assert_eq!(11, U11::to_int());
    assert_eq!(12, U12::to_int());
    assert_eq!(13, U13::to_int());
    assert_eq!(14, U14::to_int());
    assert_eq!(15, U15::to_int());
}

// Adding bits to unsigned integers ------------------------------------------------------

/// Adding the 0 bit to any `Unsigned`: `U + B0 = U`
impl<U: Unsigned> Add<B0> for U {
    type Output = U;
}
/// Adding the 1 bit to a `UTerm`: `UTerm + B1 = UInt<UTerm, B1>`
impl Add<B1> for UTerm {
    type Output = UInt<UTerm, B1>;
}
/// Adding the 1 bit to a `UInt` with final bit 0: `UInt<U, B0> + B1 = UInt<U + B1>`
impl<U: Unsigned> Add<B1> for UInt<U, B0> {
    type Output = UInt<U, B1>;
}
/// Adding the 1 bit to a `UInt` with final bit 1: `UInt<U, B1> + B1 = UInt<U + B1, B0>`
impl<U: Unsigned> Add<B1> for UInt<U, B1> where U: Add<B1>, <U as Add<B1>>::Output: Unsigned {
    type Output = UInt<<U as Add<B1>>::Output, B0>;
}

#[test]
fn add_bits_to_uints() {
    type Test8 = <U7 as Add<B1>>::Output;
    type Test9 = <U8 as Add<B1>>::Output;
    type Test10 = <U9 as Add<B1>>::Output;
    type Test11 = <U10 as Add<B1>>::Output;
    type Test12 = <U11 as Add<B1>>::Output;
    type Test13 = <U12 as Add<B1>>::Output;
    type Test14 = <U13 as Add<B1>>::Output;
    type Test15 = <U14 as Add<B1>>::Output;
    type Test16 = <U15 as Add<B1>>::Output;

    type Test17 = <U17 as Add<B0>>::Output;

    assert_eq!(8, <Test8 as Unsigned>::to_int());
    assert_eq!(9, <Test9 as Unsigned>::to_int());
    assert_eq!(10, <Test10 as Unsigned>::to_int());
    assert_eq!(11, <Test11 as Unsigned>::to_int());
    assert_eq!(12, <Test12 as Unsigned>::to_int());
    assert_eq!(13, <Test13 as Unsigned>::to_int());
    assert_eq!(14, <Test14 as Unsigned>::to_int());
    assert_eq!(15, <Test15 as Unsigned>::to_int());
    assert_eq!(16, <Test16 as Unsigned>::to_int());
    assert_eq!(17, <Test17 as Unsigned>::to_int());
}
// Adding unsigned integers --------------------------------------------------------------

/// Adding `UTerm` to `UTerm`: `UTerm + UTerm = UTerm`
impl Add<UTerm> for UTerm {
    type Output = UTerm;
}
/// Adding `UInt` to `UTerm`: `UTerm + UInt<U, B> = UInt<U, B>`
impl<U: Unsigned, B: Bit> Add<UInt<U, B>> for UTerm {
    type Output = UInt<U, B>;
}
/// Adding `UTerm` to `UInt`: `UInt<U, B> + UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Add<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
}
/// Adding unsigned integers: `UInt<Ul, Bl> + UInt<Ur, Br> = UInt<Ul + (Ur + Bl & Br), Bl ^ Br>`
impl<Bl: Bit, Ul: Unsigned, Br: Bit, Ur: Unsigned> Add<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Bl: And<Br> + Xor<Br>,
          Ul: Add<<Ur as Add<<Bl as And<Br>>::Output>>::Output>,
          Ur: Add<<Bl as And<Br>>::Output>,
          <Ul as Add<<Ur as Add<<Bl as And<Br>>::Output>>::Output>>::Output: Unsigned,
          <Bl as Xor<Br>>::Output: Bit
{
    type Output = UInt<
        <Ul as Add<<Ur as Add<<Bl as And<Br>>::Output>>::Output>>::Output,
        <Bl as Xor<Br>>::Output>;
}

#[test]
fn add_uints() {
    type Test0 = <U0 as Add<U0>>::Output;
    assert_eq!(0, <Test0 as Unsigned>::to_int());
    type Test1 = <U1 as Add<U0>>::Output;
    assert_eq!(1, <Test1 as Unsigned>::to_int());
    type Test1b = <U0 as Add<U1>>::Output;
    assert_eq!(1, <Test1b as Unsigned>::to_int());
    type Test2 = <U1 as Add<U1>>::Output;
    assert_eq!(2, <Test2 as Unsigned>::to_int());
    type Test2b = <U2 as Add<U0>>::Output;
    assert_eq!(2, <Test2b as Unsigned>::to_int());
    type Test2c = <U0 as Add<U2>>::Output;
    assert_eq!(2, <Test2c as Unsigned>::to_int());
    type Test3 = <U2 as Add<U1>>::Output;
    assert_eq!(3, <Test3 as Unsigned>::to_int());
    type Test3b = <U1 as Add<U2>>::Output;
    assert_eq!(3, <Test3b as Unsigned>::to_int());
    type Test4 = <U2 as Add<U2>>::Output;
    assert_eq!(4, <Test4 as Unsigned>::to_int());


    type Test62 = <U31 as Add<U31>>::Output;
    assert_eq!(62, <Test62 as Unsigned>::to_int());

    assert_eq!(32, <U32 as Unsigned>::to_int());
    assert_eq!(64, <U64 as Unsigned>::to_int());
    assert_eq!(128, <U128 as Unsigned>::to_int());
    assert_eq!(256, <U256 as Unsigned>::to_int());
    // assert_eq!(512, <U512 as Unsigned>::to_int());
    // assert_eq!(1024, <U1024 as Unsigned>::to_int());
    // assert_eq!(2048, <U2048 as Unsigned>::to_int());
    // assert_eq!(4096, <U4096 as Unsigned>::to_int());
    // assert_eq!(8192, <U8192 as Unsigned>::to_int());

}

// Subtracting bits from unsigned integers -----------------------------------------------

/// Subtracting the 0 bit from any `Unsigned`: `U - B0 = U`
impl<U: Unsigned> Sub<B0> for U {
    type Output = U;
}
/// Subtracting the 1 bit from a `UInt` with final bit 1: `UInt<U, B1> - B1 = UInt<U, B0>`
impl<U: Unsigned> Sub<B1> for UInt<U, B1> {
    type Output = UInt<U, B0>;
}
/// Subtracting the 1 bit from a `UInt` with final bit 0: `UInt<U, B0> - B1 = UInt<U - B1, B1>`
impl<U: Unsigned> Sub<B1> for UInt<U, B0> where U:Sub<B1>, <U as Sub<B1>>::Output: Unsigned {
    type Output = UInt<<U as Sub<B1>>::Output, B1>;
}

#[test]
fn sub_bits_from_uints() {
    // Uncomment for error
    // type TestN1 = <U0 as Sub<B1>>::Output;
    // assert_eq!(-1, <TestN1 as Unsigned>::to_int());

    type Test7 = <U8 as Sub<B1>>::Output;
    type Test8 = <U9 as Sub<B1>>::Output;
    type Test9 = <U10 as Sub<B1>>::Output;
    type Test10 = <U11 as Sub<B1>>::Output;
    type Test11 = <U12 as Sub<B1>>::Output;
    type Test12 = <U13 as Sub<B1>>::Output;
    type Test13 = <U14 as Sub<B1>>::Output;
    type Test14 = <U15 as Sub<B1>>::Output;
    type Test15 = <U16 as Sub<B1>>::Output;

    type Test17 = <U17 as Sub<B0>>::Output;

    assert_eq!(7, <Test7 as Unsigned>::to_int());
    assert_eq!(8, <Test8 as Unsigned>::to_int());
    assert_eq!(9, <Test9 as Unsigned>::to_int());
    assert_eq!(10, <Test10 as Unsigned>::to_int());
    assert_eq!(11, <Test11 as Unsigned>::to_int());
    assert_eq!(12, <Test12 as Unsigned>::to_int());
    assert_eq!(13, <Test13 as Unsigned>::to_int());
    assert_eq!(14, <Test14 as Unsigned>::to_int());
    assert_eq!(15, <Test15 as Unsigned>::to_int());

    assert_eq!(17, <Test17 as Unsigned>::to_int());
}

// Subtracting unsigned integers ---------------------------------------------------------

/// A trait used to determine when to borrow for subtraction.
trait Borrow<Rhs> {
    type Output;
}
/// We only borrow in this case; when we have `B0 - B1`
impl Borrow<B1> for B0 {
    type Output = B1;
}
/// We do not borrow in this case.
impl Borrow<B1> for B1 {
    type Output = B0;
}
/// We do not borrow in this case.
impl Borrow<B0> for B1 {
    type Output = B0;
}
/// We do not borrow in this case.
impl Borrow<B0> for B0 {
    type Output = B0;
}

/// Subtracting `UTerm` from `UTerm`: `UTerm - UTerm = UTerm`
impl Sub<UTerm> for UTerm {
    type Output = UTerm;
}
/// Subtracting `UTerm` from `UInt`: `UInt<U, B> - UTerm = UInt<U, B>`
impl<U, B> Sub<UTerm> for UInt<U, B> where U: Unsigned, B: Bit {
    type Output = UInt<U, B>;
}
/// Subtracting unsigned integers:
/// `UInt<Ul, Bl> - UInt<Ur, Br> = UInt<(Ul - Bl Borrow Br) - Ur, Bl ^ Br>`
/// where `Borrow` is a trait operation that only returns `B1` when
/// we need to borrow; `Bl = 0` and `Br = 1`. The rest of the time it returns `B0`.
impl<Bl: Bit, Ul: Unsigned, Br: Bit, Ur: Unsigned> Sub<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Bl: Xor<Br> + Borrow<Br>,
          Ul: Sub<<Bl as Borrow<Br>>::Output>,
          <Ul as Sub<<Bl as Borrow<Br>>::Output>>::Output: Sub<Ur>,
          <<Ul as Sub<<Bl as Borrow<Br>>::Output>>::Output as Sub<Ur>>::Output: Unsigned,
          <Bl as Xor<Br>>::Output: Bit
{
    type Output = UInt<
        <<Ul as Sub<<Bl as Borrow<Br>>::Output>>::Output as Sub<Ur>>::Output,
        <Bl as Xor<Br>>::Output>;
}
#[test]
fn sub_uints() {
    // Uncomment for error:
    // type TestN1 = <U0 as Sub<U1>>::Output;
    // assert_eq!(-1, <TestN1 as Unsigned>::to_int());

    type Test0 = <U0 as Sub<U0>>::Output;
    assert_eq!(0, <Test0 as Unsigned>::to_int());
    type Test1 = <U1 as Sub<U0>>::Output;
    assert_eq!(1, <Test1 as Unsigned>::to_int());
    type Test0b = <U1 as Sub<U1>>::Output;
    assert_eq!(0, <Test0b as Unsigned>::to_int());
    type Test2 = <U2 as Sub<U0>>::Output;
    assert_eq!(2, <Test2 as Unsigned>::to_int());
    type Test1b = <U2 as Sub<U1>>::Output;
    assert_eq!(1, <Test1b as Unsigned>::to_int());
    type Test0c = <U2 as Sub<U2>>::Output;
    assert_eq!(0, <Test0c as Unsigned>::to_int());


    type Test32 = <U64 as Sub<U32>>::Output;
    assert_eq!(32, <Test32 as Unsigned>::to_int());

    type Test0d = <U31 as Sub<U31>>::Output;
    assert_eq!(0, <Test0d as Unsigned>::to_int());

    type Test1c = <U32 as Sub<U31>>::Output;
    assert_eq!(1, <Test1c as Unsigned>::to_int());
}
