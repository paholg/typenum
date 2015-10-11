
use std::marker::PhantomData;

use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul};
use ::{NonZero, Same, Ord, Greater, Equal, Less, Cmp, SizeOf};
use ::bit::{Bit, B0, B1};
use ::__private::{Trim, PrivateAnd, PrivateXor, PrivateSub, PrivateCmp, PrivateSizeOf, PrivateDiv};

pub use ::consts::uints::{
    U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14,
    U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31,
    U32, U33, U34, U35, U36, U37, U38, U39, U40, U41, U42, U43, U44, U45, U46, U47, U48,
    U49, U50, U51, U52, U53, U54, U55, U56, U57, U58, U59, U60, U61, U62, U63, U64, U65,
    U66, U67, U68, U69, U70, U71, U72, U73, U74, U75, U76, U77, U78, U79, U80, U81, U82,
    U83, U84, U85, U86, U87, U88, U89, U90, U91, U92, U93, U94, U95, U96, U97, U98, U99,
    U100, U101, U102, U103, U104, U105, U106, U107, U108, U109, U110, U111, U112, U113,
    U114, U115, U116, U117, U118, U119, U120, U121, U122, U123, U124, U125, U126, U127,
    U128, U256, U512, U1024, U2048, U4096, U8192, U10000, U16384, U32768, U65536,

    U131072, U262144, U524288, U1048576, U2097152, U4194304, U8388608, U16777216, U33554432,
    U67108864, U134217728, U268435456, U536870912, U1073741824, U2147483648, U4294967296,
    U8589934592, U17179869184, U34359738368, U68719476736, U137438953472, U274877906944,
    U549755813888, U1099511627776, U2199023255552, U4398046511104, U8796093022208,
    U17592186044416, U35184372088832, U70368744177664, U140737488355328, U281474976710656,
    U562949953421312, U1125899906842624, U2251799813685248, U4503599627370496,
    U9007199254740992, U18014398509481984, U36028797018963968, U72057594037927936,
    U144115188075855872, U288230376151711744, U576460752303423488, U1152921504606846976,
    U2305843009213693952, U4611686018427387904, U9223372036854775808
};

/// This trait is implemented for the all things that a `UInt` can take as a parameter,
/// which is just `UInt` and `UTerm` (used to terminate the `UInt`). It should not be
/// implemented for anything outside this crate.
pub trait Unsigned {
    fn to_u8() -> u8;
    fn to_u16() -> u16;
    fn to_u32() -> u32;
    fn to_u64() -> u64;
    fn to_usize() -> usize;

    fn to_i8() -> i8;
    fn to_i16() -> i16;
    fn to_i32() -> i32;
    fn to_i64() -> i64;
    fn to_isize() -> isize;
}

/// The terminating type for `UInt`, it always comes after the most significant bit.
pub struct UTerm;

impl Unsigned for UTerm {
    fn to_u8() -> u8 { 0 }
    fn to_u16() -> u16 { 0 }
    fn to_u32() -> u32 { 0 }
    fn to_u64() -> u64 { 0 }
    fn to_usize() -> usize { 0 }

    fn to_i8() -> i8 { 0 }
    fn to_i16() -> i16 { 0 }
    fn to_i32() -> i32 { 0 }
    fn to_i64() -> i64 { 0 }
    fn to_isize() -> isize { 0 }
}

/// UInt is defined recursevly, where B is the least significant bit and U is the rest
/// of the number. U can be another UInt or UTerm. In order to keep numbers unique, leading
/// zeros are not allowed, so `UInt<UTerm, B0>` should never show up anywhere ever.
pub struct UInt<U, B> {
    _marker: PhantomData<(U, B)>
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {
    fn to_u8() -> u8 { B::to_u8() | U::to_u8() << 1 }
    fn to_u16() -> u16 { B::to_u8() as u16 | U::to_u16() << 1 }
    fn to_u32() -> u32 { B::to_u8() as u32 | U::to_u32() << 1 }
    fn to_u64() -> u64 { B::to_u8() as u64 | U::to_u64() << 1 }
    fn to_usize() -> usize { B::to_u8() as usize | U::to_usize() << 1 }

    fn to_i8() -> i8 { B::to_u8() as i8 | U::to_i8() << 1 }
    fn to_i16() -> i16 { B::to_u8() as i16 | U::to_i16() << 1 }
    fn to_i32() -> i32 { B::to_u8() as i32 | U::to_i32() << 1 }
    fn to_i64() -> i64 { B::to_u8() as i64 | U::to_i64() << 1 }
    fn to_isize() -> isize { B::to_u8() as isize | U::to_isize() << 1 }
}

impl<U: Unsigned, B: Bit> NonZero for UInt<U, B> {}

impl<U: Unsigned> Same<U> for U {
    type Output = U;
}

#[test]
fn confirm_uints() {
    assert_eq!(0, U0::to_u64());
    assert_eq!(1, U1::to_u64());
    assert_eq!(2, U2::to_u64());
    assert_eq!(3, U3::to_u64());
    assert_eq!(4, U4::to_u64());
    assert_eq!(5, U5::to_u64());
    assert_eq!(6, U6::to_u64());
    assert_eq!(7, U7::to_u64());
    assert_eq!(8, U8::to_u64());
    assert_eq!(9, U9::to_u64());
    assert_eq!(10, U10::to_u64());
    assert_eq!(11, U11::to_u64());
    assert_eq!(12, U12::to_u64());
    assert_eq!(13, U13::to_u64());
    assert_eq!(14, U14::to_u64());
    assert_eq!(15, U15::to_u64());

    assert_eq!(10000, U10000::to_u64());
}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_uint_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_u64(), <Test as Unsigned>::to_u64());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_u64(), <Test as Unsigned>::to_u64());
        }
        );
}

// Getting size of unsigned integers -----------------------------------------------------

/// Size of `UTerm` by itself is 1
impl SizeOf for UTerm {
    type Output = U1;
}

/// Size of a `UInt`
impl<U: Unsigned, B: Bit> SizeOf for UInt<U, B>
    where UInt<U, B>: PrivateSizeOf
{
    type Output = <UInt<U, B> as PrivateSizeOf>::Output;
}

/// Size of `UTerm` inside a number is 0
impl PrivateSizeOf for UTerm {
    type Output = U0;
}

/// Size of bit is 1
impl<U: Unsigned, B: Bit> PrivateSizeOf for UInt<U, B>
    where U: PrivateSizeOf,
    <U as PrivateSizeOf>::Output: Add<B1>
{
    type Output = <<U as PrivateSizeOf>::Output as Add<B1>>::Output;
}

#[test]
fn sizeof_uints() {
    test_uint_op!(SizeOf U0 = U1);
    test_uint_op!(SizeOf U1 = U1);
    test_uint_op!(SizeOf U2 = U2);
    test_uint_op!(SizeOf U3 = U2);
    test_uint_op!(SizeOf U4 = U3);
    test_uint_op!(SizeOf U127 = U7);
    test_uint_op!(SizeOf U128 = U8);
}


// Adding bits to unsigned integers ------------------------------------------------------

/// `UTerm + B0 = UTerm`
impl Add<B0> for UTerm {
    type Output = UTerm;
    fn add(self, _: B0) -> Self::Output { unreachable!() }
}
/// `UInt + B0 = UInt`
impl<U: Unsigned, B: Bit> Add<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn add(self, _: B0) -> Self::Output { unreachable!() }
}
/// `UTerm + B1 = UInt<UTerm, B1>`
impl Add<B1> for UTerm {
    type Output = UInt<UTerm, B1>;
    fn add(self, _: B1) -> Self::Output { unreachable!() }
}
/// `UInt<U, B0> + B1 = UInt<U + B1>`
impl<U: Unsigned> Add<B1> for UInt<U, B0> {
    type Output = UInt<U, B1>;
    fn add(self, _: B1) -> Self::Output { unreachable!() }
}
/// `UInt<U, B1> + B1 = UInt<U + B1, B0>`
impl<U: Unsigned> Add<B1> for UInt<U, B1> where U: Add<B1>, <U as Add<B1>>::Output: Unsigned {
    type Output = UInt<<U as Add<B1>>::Output, B0>;
    fn add(self, _: B1) -> Self::Output { unreachable!() }
}

#[test]
fn add_bits_to_uints() {
    test_uint_op!(U0 Add B1 = U1);
    test_uint_op!(U1 Add B1 = U2);
    test_uint_op!(U7 Add B1 = U8);
    test_uint_op!(U7 Add B0 = U7);
    test_uint_op!(U16 Add B0 = U16);

    test_uint_op!(U65536 Add B0 = U65536);
}
// Adding unsigned integers --------------------------------------------------------------

/// `UTerm + UTerm = UTerm`
impl Add<UTerm> for UTerm {
    type Output = UTerm;
    fn add(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// `UTerm + UInt<U, B> = UInt<U, B>`
impl<U: Unsigned, B: Bit> Add<UInt<U, B>> for UTerm {
    type Output = UInt<U, B>;
    fn add(self, _: UInt<U, B>) -> Self::Output { unreachable!() }
}

/// `UInt<U, B> + UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Add<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn add(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B0> + UInt<Ur, B0> = UInt<Ul + Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: Add<Ur> {
    type Output = UInt<<Ul as Add<Ur>>::Output, B0>;
    fn add(self, _:UInt<Ur, B0>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B0> + UInt<Ur, B1> = UInt<Ul + Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B1>> for UInt<Ul, B0> where Ul: Add<Ur> {
    type Output = UInt<<Ul as Add<Ur>>::Output, B1>;
    fn add(self, _:UInt<Ur, B1>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B1> + UInt<Ur, B0> = UInt<Ul + Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: Add<Ur> {
    type Output = UInt<<Ul as Add<Ur>>::Output, B1>;
    fn add(self, _:UInt<Ur, B0>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B1> + UInt<Ur, B1> = UInt<(Ul + Ur) + B1, B0>`
impl<Ul: Unsigned, Ur: Unsigned> Add<UInt<Ur, B1>> for UInt<Ul, B1>
    where Ul: Add<Ur>,
          <Ul as Add<Ur>>::Output: Add<B1>
{
    type Output = UInt<<<Ul as Add<Ur>>::Output as Add<B1>>::Output, B0>;
    fn add(self, _:UInt<Ur, B1>) -> Self::Output { unreachable!() }
}

#[test]
fn add_uints() {
    test_uint_op!(U0 Add U0 = U0);
    test_uint_op!(U1 Add U0 = U1);
    test_uint_op!(U0 Add U1 = U1);
    test_uint_op!(U1 Add U1 = U2);

    test_uint_op!(U7 Add U2 = U9);
    test_uint_op!(U31 Add U31 = U62);
    test_uint_op!(U32 Add U31 = U63);
    test_uint_op!(U31 Add U32 = U63);

    test_uint_op!(U0 Add U32 = U32);
    test_uint_op!(U32 Add U0 = U32);

    test_uint_op!(U32768 Add U32768 = U65536);
}

// Subtracting bits from unsigned integers -----------------------------------------------

/// `UTerm - B0 = Term`
impl Sub<B0> for UTerm {
    type Output = UTerm;
    fn sub(self, _:B0) -> Self::Output { unreachable!() }
}

/// `UInt - B0 = UInt`
impl<U: Unsigned, B: Bit> Sub<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn sub(self, _:B0) -> Self::Output { unreachable!() }
}
/// `UInt<U, B1> - B1 = UInt<U, B0>`
impl<U: Unsigned, B: Bit> Sub<B1> for UInt<UInt<U, B>, B1> {
    type Output = UInt<UInt<U, B>, B0>;
    fn sub(self, _:B1) -> Self::Output { unreachable!() }
}

/// `UInt<UTerm, B1> - B1 = UTerm`
impl Sub<B1> for UInt<UTerm, B1> {
    type Output = UTerm;
    fn sub(self, _:B1) -> Self::Output { unreachable!() }
}

/// `UInt<U, B0> - B1 = UInt<U - B1, B1>`
impl<U: Unsigned> Sub<B1> for UInt<U, B0> where U:Sub<B1>, <U as Sub<B1>>::Output: Unsigned {
    type Output = UInt<<U as Sub<B1>>::Output, B1>;
    fn sub(self, _:B1) -> Self::Output { unreachable!() }
}

#[test]
fn sub_bits_from_uints() {
    // Uncomment for error
    //test_uint_op!(U0 Sub B1 = U0);

    test_uint_op!(U0 Sub B0 = U0);
    test_uint_op!(U127 Sub B0 = U127);
    test_uint_op!(U128 Sub B0 = U128);

    test_uint_op!(U8 Sub B1 = U7);
    test_uint_op!(U9 Sub B1 = U8);
    test_uint_op!(U10 Sub B1 = U9);
    test_uint_op!(U128 Sub B1 = U127);
    test_uint_op!(U127 Sub B1 = U126);
}

// Subtracting unsigned integers ---------------------------------------------------------
/// `UTerm - UTerm = UTerm`
impl Sub<UTerm> for UTerm {
    type Output = UTerm;
    fn sub(self, _:UTerm) -> Self::Output { unreachable!() }
}
/// Subtracting unsigned integers. We just do our `PrivateSub` and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> Sub<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateSub<Ur>,
          <UInt<Ul, Bl> as PrivateSub<Ur>>::Output: Trim
{
    type Output = <<UInt<Ul, Bl> as PrivateSub<Ur>>::Output as Trim>::Output;
    fn sub(self, _:Ur) -> Self::Output { unreachable!() }
}

/// `U - UTerm = U`
impl<U: Unsigned> PrivateSub<UTerm> for U {
    type Output = U;
}

/// `UInt<Ul, B0> - UInt<Ur, B0> = UInt<Ul - Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B0>> for UInt<Ul, B0>
    where Ul: PrivateSub<Ur>
{
    type Output = UInt<<Ul as PrivateSub<Ur>>::Output, B0>;
}

/// `UInt<Ul, B0> - UInt<Ur, B1> = UInt<(Ul - Ur) - B1, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B1>> for UInt<Ul, B0>
    where Ul: PrivateSub<Ur>,
<Ul as PrivateSub<Ur>>::Output: Sub<B1>
{
    type Output = UInt<<<Ul as PrivateSub<Ur>>::Output as Sub<B1>>::Output, B1>;
}

/// `UInt<Ul, B1> - UInt<Ur, B0> = UInt<Ul - Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B0>> for UInt<Ul, B1>
    where Ul: PrivateSub<Ur>
{
    type Output = UInt<<Ul as PrivateSub<Ur>>::Output, B1>;
}

/// `UInt<Ul, B1> - UInt<Ur, B1> = UInt<Ul - Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B1>> for UInt<Ul, B1>
    where Ul: PrivateSub<Ur>
{
    type Output = UInt<<Ul as PrivateSub<Ur>>::Output, B0>;
}


#[test]
fn sub_uints() {
    // Uncomment for error:
    // test_uint_op!(U0 Sub U1 = U0);

    test_uint_op!(U0 Sub U0 = U0);
    test_uint_op!(U1 Sub U0 = U1);
    test_uint_op!(U1 Sub U1 = U0);
    test_uint_op!(U2 Sub U0 = U2);
    test_uint_op!(U2 Sub U1 = U1);
    test_uint_op!(U2 Sub U2 = U0);

    test_uint_op!(U64 Sub U32 = U32);
    test_uint_op!(U31 Sub U31 = U0);

    test_uint_op!(U32 Sub U31 = U1);

    test_uint_op!(U65536 Sub U65536 = U0);
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
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B0>> for UInt<Ul, B0>
    where Ul: PrivateAnd<Ur>
{
    type Output = UInt<<Ul as PrivateAnd<Ur>>::Output, B0>;
}

/// `UInt<Ul, B0> & UInt<Ur, B1> = UInt<Ul & Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B1>> for UInt<Ul, B0>
    where Ul: PrivateAnd<Ur>
{
    type Output = UInt<<Ul as PrivateAnd<Ur>>::Output, B0>;
}

/// `UInt<Ul, B1> & UInt<Ur, B0> = UInt<Ul & Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B0>> for UInt<Ul, B1>
    where Ul: PrivateAnd<Ur>
{
    type Output = UInt<<Ul as PrivateAnd<Ur>>::Output, B0>;
}

/// `UInt<Ul, B1> & UInt<Ur, B1> = UInt<Ul & Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateAnd<UInt<Ur, B1>> for UInt<Ul, B1>
    where Ul: PrivateAnd<Ur>
{
    type Output = UInt<<Ul as PrivateAnd<Ur>>::Output, B1>;
}

impl<Ur: Unsigned> BitAnd<Ur> for UTerm {
    type Output = UTerm;
    fn bitand(self, _: Ur) -> Self::Output { unreachable!() }
}

/// Anding unsigned integers.
/// We use our `PrivateAnd` operator and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> BitAnd<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateAnd<Ur>,
          <UInt<Ul, Bl> as PrivateAnd<Ur>>::Output: Trim
{
    type Output = <<UInt<Ul, Bl> as PrivateAnd<Ur>>::Output as Trim>::Output;
    fn bitand(self, _: Ur) -> Self::Output { unreachable!() }
}

#[test]
fn and_uints() {
    test_uint_op!(U0 BitAnd U0 = U0);
    test_uint_op!(U1 BitAnd U0 = U0);
    test_uint_op!(U0 BitAnd U1 = U0);
    test_uint_op!(U1 BitAnd U1 = U1);

    test_uint_op!(U2 BitAnd U9 = U0);
    test_uint_op!(U9 BitAnd U2 = U0);
    test_uint_op!(U127 BitAnd U128 = U0);
    test_uint_op!(U3 BitAnd U7 = U3);
    test_uint_op!(U15 BitAnd U15 = U15);

    test_uint_op!(U120 BitAnd U105 = U104);

    test_uint_op!(U65536 BitAnd U65536 = U65536);
}

/// `UTerm | X = X`
impl<U: Unsigned> BitOr<U> for UTerm {
    type Output = U;
    fn bitor(self, _: U) -> Self::Output { unreachable!() }
}
///  `X | UTerm = X`
impl<B: Bit, U: Unsigned> BitOr<UTerm> for UInt<U, B> {
    type Output = Self;
    fn bitor(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B0> | UInt<Ur, B0> = UInt<Ul | Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B0>> for UInt<Ul, B0> where Ul: BitOr<Ur> {
    type Output = UInt<<Ul as BitOr<Ur>>::Output, B0>;
    fn bitor(self, _: UInt<Ur, B0>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B0> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B1>> for UInt<Ul, B0> where Ul: BitOr<Ur> {
    type Output = UInt<<Ul as BitOr<Ur>>::Output, B1>;
    fn bitor(self, _: UInt<Ur, B1>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B1> | UInt<Ur, B0> = UInt<Ul | Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B0>> for UInt<Ul, B1> where Ul: BitOr<Ur> {
    type Output = UInt<<Ul as BitOr<Ur>>::Output, B1>;
    fn bitor(self, _: UInt<Ur, B0>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B1> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> BitOr<UInt<Ur, B1>> for UInt<Ul, B1> where Ul: BitOr<Ur> {
    type Output = UInt<<Ul as BitOr<Ur>>::Output, B1>;
    fn bitor(self, _: UInt<Ur, B1>) -> Self::Output { unreachable!() }
}

#[test]
fn or_uints() {
    test_uint_op!(U0 BitOr U0 = U0);
    test_uint_op!(U1 BitOr U0 = U1);
    test_uint_op!(U0 BitOr U1 = U1);
    test_uint_op!(U1 BitOr U1 = U1);


    test_uint_op!(U2 BitOr U9 = U11);
    test_uint_op!(U3 BitOr U7 = U7);

    test_uint_op!(U15 BitOr U15 = U15);

    test_uint_op!(U65536 BitOr U65536 = U65536);
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
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B0>> for UInt<Ul, B0>
    where Ul: PrivateXor<Ur>
{
    type Output = UInt<<Ul as PrivateXor<Ur>>::Output, B0>;
}

/// `UInt<Ul, B0> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B1>> for UInt<Ul, B0>
    where Ul: PrivateXor<Ur>
{
    type Output = UInt<<Ul as PrivateXor<Ur>>::Output, B1>;
}

/// `UInt<Ul, B1> ^ UInt<Ur, B0> = UInt<Ul ^ Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B0>> for UInt<Ul, B1>
    where Ul: PrivateXor<Ur>
{
    type Output = UInt<<Ul as PrivateXor<Ur>>::Output, B1>;
}

/// `UInt<Ul, B1> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateXor<UInt<Ur, B1>> for UInt<Ul, B1>
    where Ul: PrivateXor<Ur>
{
    type Output = UInt<<Ul as PrivateXor<Ur>>::Output, B0>;
}

/// 0 ^ X = X
impl<Ur: Unsigned> BitXor<Ur> for UTerm {
    type Output = Ur;
    fn bitxor(self, _: Ur) -> Self::Output { unreachable!() }
}
/// Xoring unsigned integers.
/// We use our `PrivateXor` operator and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> BitXor<Ur> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateXor<Ur>,
          <UInt<Ul, Bl> as PrivateXor<Ur>>::Output: Trim
{
    type Output = <<UInt<Ul, Bl> as PrivateXor<Ur>>::Output as Trim>::Output;
    fn bitxor(self, _: Ur) -> Self::Output { unreachable!() }
}

#[test]
fn xor_uints() {
    test_uint_op!(U0 BitXor U0 = U0);
    test_uint_op!(U1 BitXor U0 = U1);
    test_uint_op!(U0 BitXor U1 = U1);
    test_uint_op!(U1 BitXor U1 = U0);

    test_uint_op!(U2 BitXor U9 = U11);
    test_uint_op!(U3 BitXor U7 = U4);

    test_uint_op!(U15 BitXor U15 = U0);

    test_uint_op!(U65536 BitXor U65536 = U0);
}

/// Shifting left `UTerm` by an unsigned integer: `UTerm << U = UTerm`
impl<U: Unsigned> Shl<U> for UTerm {
    type Output = UTerm;
    fn shl(self, _: U) -> Self::Output { unreachable!() }
}

/// Shifting left `UInt` by `UTerm`: `UInt<U, B> << UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Shl<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shl(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// Shifting left any unsigned by a zero bit: `U << B0 = U`
impl<U: Unsigned, B: Bit> Shl<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shl(self, _: B0) -> Self::Output { unreachable!() }
}

/// Shifting UTerm by a zero bit: `UTerm << B0 = UTerm`
impl Shl<B0> for UTerm {
    type Output = UTerm;
    fn shl(self, _: B0) -> Self::Output { unreachable!() }
}

/// Shifting left a `UInt` by a one bit: `UInt<U, B> << B1 = UInt<UInt<U, B>, B0>`
impl<U: Unsigned, B: Bit> Shl<B1> for UInt<U, B> {
    type Output = UInt<UInt<U, B>, B0>;
    fn shl(self, _: B1) -> Self::Output { unreachable!() }
}

/// Shifting left a `UTerm` by a 1 bit: `UTerm << B1 = UTerm`
impl Shl<B1> for UTerm {
    type Output = UTerm;
    fn shl(self, _: B1) -> Self::Output { unreachable!() }
}

/// Shifting left `UInt` by `UInt`: `X << Y` = `UInt(X, B0) << (Y - 1)`
impl<U: Unsigned, B: Bit, Ur: Unsigned, Br: Bit> Shl<UInt<Ur, Br>> for UInt<U, B>
where UInt<Ur, Br> : Sub<B1>,
    UInt<UInt<U, B>, B0> : Shl<<UInt<Ur, Br> as Sub<B1>>::Output>
{
    type Output =
        <
            UInt<UInt<U, B>, B0> as Shl<
                    <UInt<Ur, Br> as Sub<B1>>::Output
                >
        >::Output;
        fn shl(self, _: UInt<Ur, Br>) -> Self::Output { unreachable!() }
}

#[test]
fn shl_tests() {
    test_uint_op!(U0 Shl B0 = U0);
    test_uint_op!(U0 Shl B1 = U0);

    test_uint_op!(U1 Shl B0 = U1);
    test_uint_op!(U1 Shl B1 = U2);

    test_uint_op!(U0 Shl U0 = U0);
    test_uint_op!(U1 Shl U0 = U1);
    test_uint_op!(U0 Shl U1 = U0);
    test_uint_op!(U1 Shl U1 = U2);

    test_uint_op!(U2 Shl U9 = U1024);
    test_uint_op!(U7 Shl U3 = U56);

    test_uint_op!(U1 Shl U15 = U32768);
}

/// Shifting right a `UTerm` by an unsigned integer: `UTerm >> U = UTerm`
impl<U: Unsigned> Shr<U> for UTerm {
    type Output = UTerm;
    fn shr(self, _: U) -> Self::Output { unreachable!() }
}

/// Shifting right `UInt` by `UTerm`: `UInt<U, B> >> UTerm = UInt<U, B>`
impl<U: Unsigned, B: Bit> Shr<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shr(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// Shifting right UTerm by a zero bit: `UTerm >> B0 = UTerm`
impl Shr<B0> for UTerm {
    type Output = UTerm;
    fn shr(self, _: B0) -> Self::Output { unreachable!() }
}

/// Shifting right any unsigned by a zero bit: `U >> B0 = U`
impl<U: Unsigned, B: Bit> Shr<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn shr(self, _: B0) -> Self::Output { unreachable!() }
}

/// Shifting right a `UInt` by a 1 bit: `UInt<U, B> >> B1 = U`
impl<U: Unsigned, B: Bit> Shr<B1> for UInt<U, B> {
    type Output = U;
    fn shr(self, _: B1) -> Self::Output { unreachable!() }
}

/// Shifting right a `UTerm` by a 1 bit: `UTerm >> B1 = UTerm`
impl Shr<B1> for UTerm {
    type Output = UTerm;
    fn shr(self, _: B1) -> Self::Output { unreachable!() }
}

/// Shifting right `UInt` by `UInt`: `UInt(U, B) >> Y` = `U >> (Y - 1)`
impl<U: Unsigned, B: Bit, Ur: Unsigned, Br: Bit> Shr<UInt<Ur, Br>> for UInt<U, B>
where UInt<Ur, Br> : Sub<B1>,
    U : Shr<<UInt<Ur, Br> as Sub<B1>>::Output>
{
    type Output = <U as Shr<<UInt<Ur, Br> as Sub<B1>>::Output>>::Output;
    fn shr(self, _: UInt<Ur, Br>) -> Self::Output { unreachable!() }
}

#[test]
fn shr_tests() {
    // test_uint_op!(U0 Shr B0 = U0);
    // test_uint_op!(U0 Shr B1 = U0);

    // test_uint_op!(U1 Shr B0 = U1);
    // test_uint_op!(U1 Shr B1 = U0);

    test_uint_op!(U0 Shr U0 = U0);
    test_uint_op!(U1 Shr U0 = U1);
    test_uint_op!(U0 Shr U1 = U0);
    test_uint_op!(U1 Shr U1 = U0);

    test_uint_op!(U9 Shr U2 = U2);
    test_uint_op!(U7 Shr U3 = U0);

    test_uint_op!(U65536 Shr U15 = U2);
}

// Multiplying unsigned integers ---------------------------------------------------------

/// `UInt * B0 = UTerm`
impl<U: Unsigned, B: Bit> Mul<B0> for UInt<U, B> {
    type Output = UTerm;
    fn mul(self, _: B0) -> Self::Output { unreachable!() }
}

/// `UTerm * B = UTerm`
impl<B: Bit> Mul<B> for UTerm {
    type Output = UTerm;
    fn mul(self, _: B) -> Self::Output { unreachable!() }
}

/// `UInt * B1 = UInt`
impl<U: Unsigned, B: Bit> Mul<B1> for UInt<U, B> {
    type Output = UInt<U, B>;
    fn mul(self, _: B1) -> Self::Output { unreachable!() }
}

/// `UInt<U, B> * UTerm = UTerm`
impl<U: Unsigned, B: Bit> Mul<UTerm> for UInt<U, B> {
    type Output = UTerm;
    fn mul(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// `UTerm * UInt<U, B> = UTerm`
impl<U: Unsigned, B: Bit> Mul<UInt<U, B>> for UTerm {
    type Output = UTerm;
    fn mul(self, _: UInt<U, B>) -> Self::Output { unreachable!() }
}

/// `UTerm * UTerm = UTerm`
impl Mul<UTerm> for UTerm {
    type Output = UTerm;
    fn mul(self, _: UTerm) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B0> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0>`
impl<Ul: Unsigned, B: Bit, Ur: Unsigned> Mul<UInt<Ur, B>> for UInt<Ul, B0>
   where Ul: Mul<UInt<Ur, B>>
{
    type Output = UInt<<Ul as Mul<UInt<Ur, B>>>::Output, B0>;
    fn mul(self, _: UInt<Ur, B>) -> Self::Output { unreachable!() }
}

/// `UInt<Ul, B1> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0> + UInt<Ur, B>`
impl<Ul: Unsigned, B: Bit, Ur: Unsigned> Mul<UInt<Ur, B>> for UInt<Ul, B1>
    where Ul: Mul<UInt<Ur, B>>,
UInt<<Ul as Mul<UInt<Ur, B>>>::Output, B0>: Add<UInt<Ur, B>>
{
    type Output = <UInt<<Ul as Mul<UInt<Ur, B>>>::Output, B0> as Add<UInt<Ur, B>>>::Output;
    fn mul(self, _: UInt<Ur, B>) -> Self::Output { unreachable!() }
}

#[test]
fn mul_tests() {
    test_uint_op!(U0 Mul U0 = U0);
    test_uint_op!(U1 Mul U0 = U0);
    test_uint_op!(U0 Mul U1 = U0);
    test_uint_op!(U1 Mul U1 = U1);
    test_uint_op!(U0 Mul B1 = U0);
    test_uint_op!(U0 Mul U2 = U0);

    test_uint_op!(U1 Mul U2 = U2);
    test_uint_op!(U2 Mul U1 = U2);
    test_uint_op!(U2 Mul U2 = U4);


    test_uint_op!(U12 Mul U5 = U60);
    test_uint_op!(U5 Mul U12 = U60);
    test_uint_op!(U15 Mul U4 = U60);
    test_uint_op!(U4 Mul U15 = U60);
    test_uint_op!(U32 Mul U8 = U256);

    test_uint_op!(U65536 Mul U1 = U65536);
    test_uint_op!(U1 Mul U65536 = U65536);

    test_uint_op!(U65536 Mul U65536 = U4294967296);
}

// Comparing unsigned integers -----------------------------------------------------------

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

impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit> Cmp<UInt<Ur, Br>> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: PrivateCmp<UInt<Ur, Br>, Equal>
{
    type Output = <UInt<Ul, Bl> as PrivateCmp<UInt<Ur, Br>, Equal>>::Output;
}

/// Comparing non-terimal bits, with both having bit B0. These are the same, so we propogate `SoFar`.
impl<Ul, Bl, Ur, Br, S> PrivateCmp<UInt<UInt<Ur, Br>, B0>, S> for UInt<UInt<Ul, Bl>, B0>
    where Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, S: Ord,
          UInt<Ul, Bl>: PrivateCmp<UInt<Ur, Br>, S>,
{
    type Output = <UInt<Ul, Bl> as PrivateCmp<UInt<Ur, Br>, S>>::Output;
}

/// Comparing non-terimal bits, with both having bit B1. These are the same, so we propogate `SoFar`.
impl<Ul, Bl, Ur, Br, S> PrivateCmp<UInt<UInt<Ur, Br>, B1>, S> for UInt<UInt<Ul, Bl>, B1>
    where Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, S: Ord,
          UInt<Ul, Bl>: PrivateCmp<UInt<Ur, Br>, S>,
{
    type Output = <UInt<Ul, Bl> as PrivateCmp<UInt<Ur, Br>, S>>::Output;
}

/// Comparing non-terimal bits, with Lhs having bit B0 and Rhs having bit B1. `SoFar`, Lhs is `Less`.
impl<Ul, Bl, Ur, Br, S> PrivateCmp<UInt<UInt<Ur, Br>, B1>, S> for UInt<UInt<Ul, Bl>, B0>
    where Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, S: Ord,
          UInt<Ul, Bl>: PrivateCmp<UInt<Ur, Br>, Less>,
{
    type Output = <UInt<Ul, Bl> as PrivateCmp<UInt<Ur, Br>, Less>>::Output;
}

/// Comparing non-terimal bits, with Lhs having bit B1 and Rhs having bit B0. `SoFar`, Lhs is `Greater`.
impl<Ul, Bl, Ur, Br, S> PrivateCmp<UInt<UInt<Ur, Br>, B0>, S> for UInt<UInt<Ul, Bl>, B1>
    where Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, S: Ord,
          UInt<Ul, Bl>: PrivateCmp<UInt<Ur, Br>, Greater>,
{
    type Output = <UInt<Ul, Bl> as PrivateCmp<UInt<Ur, Br>, Greater>>::Output;
}

/// Comparing when Rhs has finished but Lhs has not; Lhs is `Greater`.
impl<Ul, Bl1, Bl2, Br, S> PrivateCmp<UInt<UTerm, Br>, S> for UInt<UInt<Ul, Bl2>, Bl1>
    where Ul: Unsigned, Bl1: Bit, Bl2: Bit, Br: Bit, S: Ord
{
    type Output = Greater;
}

/// Comparing when Lhs has finished but Rhs has not; Lhs is `Less`.
impl<Bl, Ur, Br1, Br2, S> PrivateCmp<UInt<UInt<Ur, Br2>, Br1>, S> for UInt<UTerm, Bl>
    where Bl: Bit, Ur: Unsigned, Br1: Bit, Br2: Bit, S: Ord
{
    type Output = Less;
}

/// Comparing when both are at terminal bits and both have `B0`. Go by `SoFar`.
impl<S: Ord> PrivateCmp<UInt<UTerm, B0>, S> for UInt<UTerm, B0> {
    type Output = S;
}

/// Comparing when both are at terminal bits and both have `B1`. Go by `SoFar`.
impl<S: Ord> PrivateCmp<UInt<UTerm, B1>, S> for UInt<UTerm, B1> {
    type Output = S;
}

/// Comparing when both are at terminal bits and Lhs has `B0` while Rhs has `B1`. Lhs is `Less`.
impl<S: Ord> PrivateCmp<UInt<UTerm, B1>, S> for UInt<UTerm, B0> {
    type Output = Less;
}

/// Comparing when both are at terminal bits and Lhs has `B1` while Rhs has `B0`. Lhs is `Greater`.
impl<S: Ord> PrivateCmp<UInt<UTerm, B0>, S> for UInt<UTerm, B1> {
    type Output = Greater;
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

#[test]
fn test_ord() {
    test_ord!(U0 == U0);
    test_ord!(U1 > U0);
    test_ord!(U0 < U1);

    test_ord!(U85 > U0);
    test_ord!(U0 < U85);

    test_ord!(U2 > U1);
    test_ord!(U1 < U2);

    test_ord!(U128 > U127);
    test_ord!(U127 < U128);

    test_ord!(U125 == U125);
    test_ord!(U512 == U512);
}

// Dividing unsigned integers ---------------------------------------------------------

/// Dividing any unsigned by the 1 bit: `U / B1: Q = U; R = UTerm`
impl<U: Unsigned> PrivateDiv<B1> for U {
    type Quotient = U;
    type Remainder = UTerm;
}

/// Dividing `UTerm` by any `UInt`: `UTerm / UInt<U, B>: Q = UTerm; R = UTerm`
impl<U: Unsigned, B: Bit> PrivateDiv<UInt<U, B>> for UTerm {
    type Quotient = UTerm;
    type Remainder = UTerm;
}

/// Dividing `UInt` by `UInt`: `UInt<Ul, Bl> / UInt<Ur, Br>:
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Unsigned> PrivateDiv<UInt<Ur, Br>> for UInt<Ul, Bl> {
    type Quotient = UTerm;
    type Remainder = UTerm;
}
