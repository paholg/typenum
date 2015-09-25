
use std::marker::PhantomData;

use ::{Same, Add, And, Xor};
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
/// zeros are not allowed: `UInt<UInt<UTerm, B0>, B1>` is not allowed, and `UInt<UTerm, B1>`
/// should be used instead to represent the number 1.
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

pub type U0 = UInt<UTerm, B0>;
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

/// Adding the 0 bit to any Unsigned: U + B0 = U
impl<U> Add<B0> for U where U: Unsigned {
    type Output = U;
}
/// Adding the 1 bit to a UTerm: UTerm + B1 = UInt<UTerm, B1>
impl Add<B1> for UTerm {
    type Output = UInt<UTerm, B1>;
}
/// Adding the 1 bit to a UInt with final bit 0: UInt<U, B0> + B1 = UInt<U + B1>
impl<U> Add<B1> for UInt<U, B0> where U: Unsigned {
    type Output = UInt<U, B1>;
}
/// Adding the 1 bit to a UInt with final bit 1: UInt<U, B1> + B1 = UInt<U + B1, B0>
impl<U> Add<B1> for UInt<U, B1> where U: Unsigned + Add<B1>, <U as Add<B1>>::Output: Unsigned {
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

/// Adding UTerm to UTerm: UTerm + UTerm = UTerm
impl Add<UTerm> for UTerm {
    type Output = UTerm;
}
/// Adding UInt to UTerm: UTerm + UInt<U, B> = UInt<U, B>
impl<U, B> Add<UInt<U, B>> for UTerm where U: Unsigned, B: Bit {
    type Output = UInt<U, B>;
}
/// Adding UTerm to UInt: UInt<U, B> + UTerm = UInt<U, B>
impl<U, B> Add<UTerm> for UInt<U, B> where U: Unsigned, B: Bit {
    type Output = UInt<U, B>;
}
/// Adding unsigned integers: UInt<Ul, Bl> + UInt<Ur, Br> = UInt<Ul + (Ur + Bl & Br), Bl ^ Br>
impl<Bl, Ul, Ur, Br> Add<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Bl: Bit + And<Br> + Xor<Br>, Ul: Unsigned, Br: Bit,
          Ur: Unsigned + Add<<Bl as And<Br>>::Output>,
          Ul: Add<<Ur as Add<<Bl as And<Br>>::Output>>::Output>,
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
