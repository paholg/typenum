
use std::marker::PhantomData;

use ::{Same, Add};
use ::bit::{Bit, B0, B1};

/// This trait is implemented for the all things that a UInt can take as a parameter,
/// which is just UInt and UTerm (used to terminate the UInt). It should not be
/// implemented for anything outside this crate.
pub trait Unsigned {
    fn to_int() -> u64;
}

/// The terminating type for UInt, it always comes after the most significant bit.
pub struct UTerm;

impl Unsigned for UTerm {
    fn to_int() -> u64 { 0 }
}

/// UInt is defined recursevly, where B is the least significant bit and U is the rest
/// of the number. U can be another UInt or UTerm. In order to keep numbers unique, leading
/// zeros are not allowed: UInt<UInt<UTerm, B0>, B1> is not allowed, and UInt<UTerm, B1>
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




// /// Adding the 1 bit to a terminating UInt with final bit 1: UInt<(), B1> + B1 = UInt<UInt<(), B1>, B0>
// impl Add<B1> for UInt<UTerm, B1> {
//     type Output = UInt<UInt<UTerm, B1>, B0>;
// }
// /// Adding the 1 bit to a non-terminating UInt with final bit 1, and next bit 0: UInt<UInt<U, B0>, B1> + B1 = UInt<UInt<U, B1>, B0>
// impl<U: Unsigned> Add<B1> for UInt<UInt<U, B0>, B1> {
//     type Output = UInt<UInt<U, B1>, B0>;
// }
// /// Adding the 1 bit to a non-terminating UInt with final bit 1, and next bit 1: UInt<UInt<U, B1>, B1> + B1 = UInt<UInt<U + B1, B0>, B0>
// impl<U: Unsigned> Add<B1> for UInt<UInt<U, B1>, B1>
//     where U: Add<B1>, <U as Add<B1>>::Output: Unsigned
// {
//     type Output = UInt<UInt<<U as Add<B1>>::Output, B0>, B0>;
// }

#[test]
fn add_bits_to_uints() {
    type U = <U3 as Add<B0>>::Output;
    assert_eq!(3, <U as Unsigned>::to_int());

    <U3 as Same<U>>::Output::to_int();
}
// Adding unsigned integers --------------------------------------------------------------

// /// Adding unsigned integers: (Ul, Bl) + (Ur, Br) = (Ul + [Ur + Bl & Br], Bl ^ Br)
// /// Where Bl and Br are the least significant bits of the left and right sides, respectively.
// impl<Bl, Ul, Ur, Br> Add<UInt<Ur, Br>> for UInt<Ul, Bl>
//     where Bl: Bit, Ul: UInt, Br: Bit, Ur: UInt
// {
//     type Output = UInt<
//         <Ul as Add<<Ur as Add<<Bl as And<Br>>::Output>>::Output>>::Output,
//         <Bl as Xor<Br>>::Output>;
// }




