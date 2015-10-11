
use std::marker::PhantomData;

use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul, Div};
use ::{Same, Ord, Greater, Equal, Less, Cmp, SizeOf, Pow};

use ::bit::{Bit, B0, B1};
use ::__private::{Trim, PrivateAnd, PrivateXor, PrivateSub, PrivateCmp, PrivateSizeOf, LSB, BitAt};

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
pub struct UInt<U, B> {
    _marker: PhantomData<(U, B)>
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {
    fn to_int() -> u64 {
        B::to_int() as u64 + 2*(U::to_int())
    }
}

impl<U: Unsigned> Same<U> for U {
    type Output = U;
}

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

    assert_eq!(10000, U10000::to_int());
}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_uint_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_int(), <Test as Unsigned>::to_int());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_int(), <Test as Unsigned>::to_int());
        }
        );
}

// Getting size of unsigned integers -----------------------------------------------------

/// Size of `UTerm` by itself is 0
impl SizeOf for UTerm {
    type Output = U0;
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
    test_uint_op!(SizeOf U0 = U0);
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

// ---------------------------------------------------------------------------------------
// Get least significant bit
impl LSB for UTerm {
    type Output = B0;
}

impl<U: Unsigned, B: Bit> LSB for UInt<U, B> {
    type Output = B;
}

#[test]
fn uint_lsb() {
    type Test0 = <<U0 as LSB>::Output as Same<B0>>::Output;
    assert_eq!(<Test0 as Bit>::to_int(), B0::to_int());

    type Test1 = <<U1 as LSB>::Output as Same<B1>>::Output;
    assert_eq!(<Test1 as Bit>::to_int(), B1::to_int());

    type Test2 = <<U2 as LSB>::Output as Same<B0>>::Output;
    assert_eq!(<Test2 as Bit>::to_int(), B0::to_int());

    type Test9 = <<U9 as LSB>::Output as Same<B1>>::Output;
    assert_eq!(<Test1 as Bit>::to_int(), B1::to_int());
}

// ---------------------------------------------------------------------------------------
// Get bit at index

// can only get the 0th bit for `UTerm`
impl BitAt<U0> for UTerm {
    type Output = B0;
}

// getting the final bit of a `UInt`
impl<U: Unsigned, B: Bit> BitAt<U0> for UInt<U, B> {
    type Output = B;
}

// getting the non-final bit of a `UInt`
impl<U: Unsigned, Ba: Bit, Bb: Bit, UI: Unsigned, BI: Bit> BitAt<UInt<UI, BI>> for UInt<UInt<U, Bb>, Ba>
    where UInt<UI, BI>: Sub<U1>,
          UInt<U, Bb>: BitAt<<UInt<UI, BI> as Sub<U1>>::Output>
{
    type Output = <UInt<U, Bb> as BitAt<
        <UInt<UI, BI> as Sub<U1>>::Output
        >>::Output;
}

#[test]
fn uint_bitat() {
    type Test00 = <U0 as BitAt<U0>>::Output;
    assert_eq!(<Test00 as Bit>::to_int(), B0::to_int());

    type Test10 = <U1 as BitAt<U0>>::Output;
    assert_eq!(<Test10 as Bit>::to_int(), B1::to_int());

    type Test90 = <U9 as BitAt<U0>>::Output;
    assert_eq!(<Test90 as Bit>::to_int(), B1::to_int());
    type Test91 = <U9 as BitAt<U1>>::Output;
    assert_eq!(<Test91 as Bit>::to_int(), B0::to_int());
    type Test92 = <U9 as BitAt<U2>>::Output;
    assert_eq!(<Test92 as Bit>::to_int(), B0::to_int());
    type Test93 = <U9 as BitAt<U3>>::Output;
    assert_eq!(<Test93 as Bit>::to_int(), B1::to_int());

}

// ---------------------------------------------------------------------------------------
// Getting difference in number of bits

/// Gives SizeOf(Lhs) - SizeOf(Rhs)
pub trait BitDiff<Rhs> {
    type Output;
}

impl<Ul, Bl, Ur, Br> BitDiff<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit,
          Ul: BitDiff<Ur>
{
    type Output = <Ul as BitDiff<Ur>>::Output;
}

impl<Ul> BitDiff<UTerm> for Ul where Ul: Unsigned + SizeOf {
    type Output = <Ul as SizeOf>::Output;
}

#[test]
fn uint_bitdiff() {
    test_uint_op!(U0 BitDiff U0 = U0);
    test_uint_op!(U1 BitDiff U0 = U1);
    test_uint_op!(U1 BitDiff U1 = U0);

    test_uint_op!(U2 BitDiff U0 = U2);
    test_uint_op!(U2 BitDiff U1 = U1);
    test_uint_op!(U2 BitDiff U2 = U0);

    test_uint_op!(U3 BitDiff U0 = U2);
    test_uint_op!(U3 BitDiff U1 = U1);
    test_uint_op!(U3 BitDiff U2 = U0);
    test_uint_op!(U3 BitDiff U3 = U0);

    test_uint_op!(U4 BitDiff U0 = U3);
    test_uint_op!(U4 BitDiff U1 = U2);
    test_uint_op!(U4 BitDiff U2 = U1);
    test_uint_op!(U4 BitDiff U3 = U1);
    test_uint_op!(U4 BitDiff U4 = U0);
}

// ---------------------------------------------------------------------------------------
// Shifting one number until it's the size of another

/// Performs Shl on Lhs so that SizeOf(Lhs) = SizeOf(Rhs)
/// Fails if SizeOf(Lhs) > SizeOf(Rhs)
pub trait ShiftDiff<Rhs> {
    type Output;
}

impl<Ul: Unsigned, Ur: Unsigned> ShiftDiff<Ur> for Ul
    where Ur: BitDiff<Ul>,
          Ul: Shl<<Ur as BitDiff<Ul>>::Output>
{
    type Output = <Ul as Shl<<Ur as BitDiff<Ul>>::Output>>::Output;
}

#[test]
fn uint_shiftdiff() {
    test_uint_op!(U3 ShiftDiff U16 = U24);
}

// ---------------------------------------------------------------------------------------
// Powers of unsigned integers

pub trait PrivatePow<Y, N> {
    type Output;
}

impl<X: Unsigned, N: Unsigned> Pow<N> for X
    where X: PrivatePow<U1, N>
{
    type Output = <X as PrivatePow<U1, N>>::Output;
}

impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U0> for X {
    type Output = Y;
}

impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U1> for X
    where X: Mul<Y>
{
    type Output = <X as Mul<Y>>::Output;
}

// N is even
impl<Y: Unsigned, U: Unsigned, B: Bit, X: Unsigned> PrivatePow<Y, UInt<UInt<U, B>, B0>> for X
    where X: Mul, <X as Mul>::Output: PrivatePow<Y, UInt<U, B>>
{
    type Output = <<X as Mul>::Output as PrivatePow<Y, UInt<U, B>>>::Output;
}
// N is odd
impl<Y: Unsigned, U: Unsigned, B: Bit, X: Unsigned> PrivatePow<Y, UInt<UInt<U, B>, B1>> for X
    where X: Mul + Mul<Y>,
<X as Mul>::Output: PrivatePow<<X as Mul<Y>>::Output, UInt<U, B>>
{
    type Output = <<X as Mul>::Output as PrivatePow<<X as Mul<Y>>::Output, UInt<U, B>>>::Output;
}

#[test]
fn pow_uints() {
    test_uint_op!(U0 Pow U0 = U1);
    test_uint_op!(U0 Pow U1 = U0);
    test_uint_op!(U1 Pow U0 = U1);

    test_uint_op!(U0 Pow U9 = U0);
    test_uint_op!(U9 Pow U0 = U1);

    test_uint_op!(U1 Pow U1 = U1);
    test_uint_op!(U2 Pow U1 = U2);
    test_uint_op!(U3 Pow U1 = U3);

    test_uint_op!(U1 Pow U2 = U1);
    test_uint_op!(U2 Pow U2 = U4);
    test_uint_op!(U3 Pow U2 = U9);

    test_uint_op!(U5 Pow U3 = U125);

    test_uint_op!(U16 Pow U15 = U1152921504606846976);
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
//     Call PrivateDiv with Divisor >> 1, I - 1
//   if C == Equal: # Sweet, we're done early with no remainder
//     return Q + 2^I
//   if C == Greater: # Do a step and keep going
//     Q += 2^I
//     I -= 1
//     Remainder -= Divisor
//     Divisor = Divisor >> 1
//     C = Remainder.cmp(Divisor)
//     Call PrivateDiv
pub trait PrivateDiv<C, I, Q, Divisor> {
    type Output;
}

pub trait PrivateDivFirstStep<C, Divisor> {
    type Output;
}

//  -----------------------------------------
// Div
impl<Ur: Unsigned, Br: Bit> Div<UInt<Ur, Br>> for UTerm {
    type Output = UTerm;
    fn div(self, _: UInt<Ur, Br>) -> Self::Output { unreachable!() }
}

impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit> Div<UInt<Ur, Br>> for UInt<Ul, Bl>
    where UInt<Ul, Bl>: Cmp<UInt<Ur, Br>>,
          UInt<Ul, Bl>: PrivateDivFirstStep<<UInt<Ul, Bl> as Cmp<UInt<Ur, Br>>>::Output,
              UInt<Ur, Br>>
{
    type Output = <UInt<Ul, Bl> as PrivateDivFirstStep<
        <UInt<Ul, Bl> as Cmp<UInt<Ur, Br>>>::Output,
        UInt<Ur, Br>
    >>::Output;
    fn div(self, _: UInt<Ur, Br>) -> Self::Output { unreachable!() }
}

//  -----------------------------------------
// PrivateDivFirstStep
impl<Divisor: Unsigned, Numerator: Unsigned> PrivateDivFirstStep<Less, Divisor> for Numerator {
    type Output = U0;
}
impl<Divisor: Unsigned, Numerator: Unsigned> PrivateDivFirstStep<Equal, Divisor> for Numerator {
    type Output = U1;
}
impl<Divisor: Unsigned, Numerator: Unsigned> PrivateDivFirstStep<Greater, Divisor> for Numerator
    where Numerator: BitDiff<Divisor> + Cmp<<Divisor as Shl<<Numerator as BitDiff<Divisor>>::Output>>::Output>,
          Divisor: Shl<<Numerator as BitDiff<Divisor>>::Output>,
          Numerator: PrivateDiv<
              <Numerator as Cmp<<Divisor as ShiftDiff<Numerator>>::Output>>::Output,
              <Numerator as BitDiff<Divisor>>::Output,
              U0,
             <Divisor as ShiftDiff<Numerator>>::Output
          >
{
    type Output = <Numerator as PrivateDiv<
        <Numerator as Cmp<<Divisor as ShiftDiff<Numerator>>::Output>>::Output,
        <Numerator as BitDiff<Divisor>>::Output, // I
        U0, // Q
        <Divisor as ShiftDiff<Numerator>>::Output // Divisor
    >>::Output;
}

//  -----------------------------------------
// PrivateDiv with I == 0

// Remainder is too small so we're done.
impl<Q, Divisor, Remainder> PrivateDiv<Less, U0, Q, Divisor> for Remainder
    where Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned
{
    type Output = Q;
}

// Remainder is the same as divisor, so we're done. No remainder!
impl<Q, Divisor, Remainder> PrivateDiv<Equal, U0, Q, Divisor> for Remainder
    where Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          Q: Add<U1>
{
    type Output = <Q as Add<U1>>::Output;
}

// Remainder is more than the divisor; same as the Equal case, but with a remainder.
impl<Q, Divisor, Remainder> PrivateDiv<Greater, U0, Q, Divisor> for Remainder
    where Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          Q: Add<U1>
{
    type Output = <Q as Add<U1>>::Output;
}

//  -----------------------------------------
// PrivateDiv with I > 0

// Remainder is equal to the divisor. We're done! Return `Q + 2^I`
impl<Ui, Bi, Q, Divisor, Remainder> PrivateDiv<Equal, UInt<Ui, Bi>, Q, Divisor> for Remainder
    where Ui: Unsigned, Bi: Bit, Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          U1: Shl<UInt<Ui, Bi>>,
          Q: Add<<U1 as Shl<UInt<Ui, Bi>>>::Output>
{
    type Output = <Q as Add<<U1 as Shl<UInt<Ui, Bi>>>::Output>>::Output;
}

// Remainder is too small so we proceed to the next step.
impl<Ui, Bi, Q, Divisor, Remainder> PrivateDiv<Less, UInt<Ui, Bi>, Q, Divisor> for Remainder
    where Ui: Unsigned, Bi: Bit, Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          Divisor: Shr<B1>,
          Remainder: Cmp<<Divisor as Shr<B1>>::Output>,
          UInt<Ui, Bi>: Sub<U1>,
          Remainder: PrivateDiv<
              <Remainder as Cmp<<Divisor as Shr<B1>>::Output>>::Output,
              <UInt<Ui, Bi> as Sub<U1>>::Output,
              Q,
              <Divisor as Shr<B1>>::Output
          >
{
    type Output = <Remainder as PrivateDiv<
        <Remainder as Cmp<<Divisor as Shr<B1>>::Output>>::Output, // Remainder.cmp(New Divisor)
        <UInt<Ui, Bi> as Sub<U1>>::Output,
        Q,
        <Divisor as Shr<B1>>::Output
    >>::Output;
}
// Remainder is bigger than the divisor.
// We set `Q += 2^I`, `I -= 1`, `R -= D`, `D >>= 1`, `C = R.cmp(new D)` and go again
impl<Ui, Bi, Q, Divisor, Remainder> PrivateDiv<Greater, UInt<Ui, Bi>, Q, Divisor> for Remainder
    where Ui: Unsigned, Bi: Bit, Q: Unsigned, Divisor: Unsigned, Remainder: Unsigned,
          Divisor: Shr<B1>,
          Remainder: Cmp<<Divisor as Shr<B1>>::Output>,
          UInt<Ui, Bi>: Sub<U1>,
          U1: Shl<UInt<Ui, Bi>>,
          Q: Add<<U1 as Shl<UInt<Ui, Bi>>>::Output>,
          Remainder: PrivateDiv<
              <Remainder as Cmp<<Divisor as Shr<B1>>::Output>>::Output,
              <UInt<Ui, Bi> as Sub<U1>>::Output,
              <Q as Add<<U1 as Shl<UInt<Ui, Bi>>>::Output>>::Output,
              <Divisor as Shr<B1>>::Output
          >
{
    type Output = <Remainder as PrivateDiv<
        <Remainder as Cmp<<Divisor as Shr<B1>>::Output>>::Output,
    <UInt<Ui, Bi> as Sub<U1>>::Output,
    <Q as Add<<U1 as Shl<UInt<Ui, Bi>>>::Output>>::Output,
    <Divisor as Shr<B1>>::Output
        >>::Output;
}

#[test]
fn div_uints() {
    test_uint_op!(U0 Div U1 = U0);
    test_uint_op!(U1 Div U1 = U1);
    test_uint_op!(U2 Div U1 = U2);
    test_uint_op!(U127 Div U1 = U127);

    test_uint_op!(U2 Div U2 = U1);
    test_uint_op!(U4 Div U2 = U2);
    test_uint_op!(U8 Div U2 = U4);
    test_uint_op!(U16 Div U2 = U8);
    test_uint_op!(U128 Div U2 = U64);

    test_uint_op!(U14 Div U7 = U2);
    test_uint_op!(U3 Div U3 = U1);
    test_uint_op!(U9 Div U3 = U3);
    test_uint_op!(U49 Div U7 = U7);

    test_uint_op!(U16 Div U4 = U4);
    // test_uint_op!(U27 Div U3 = U9);
}

// ---------------------------------------------------------------------------------------
// Dividing unsigned integers

// // final step
// impl<RvsD, Q, R, Ul, Bl, Ur, Br> PrivateDiv<RvsD, U0, Q, R, UInt<Ur, Br>> for UInt<Ul, Bl>
//     where RvsD: Ord, Q: Unsigned, R: Unsigned, Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit
// {
//     type Output = Q;
// }

// non-final step with `R < UInt<Ur, Br>`
// impl<UI, BI, Q, R, Ul, Bl, Ur, Br> PrivateDiv<Less, UInt<UI, BI>, Q, R, UInt<Ur, Br>> for UInt<Ul, Bl>
//     where UI: Unsigned, BI: Bit, Q: Unsigned, R: Unsigned, Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit,
// UInt<Ul, Bl>: BitAt<UInt<UI, BI>>,
// <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output: Bit,
// UInt<R, <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output>: Cmp<UInt<Ur, Br>>,
// UInt<UI, BI>: Sub<U1>,
// UInt<Ul, Bl>: PrivateDiv<        <UInt<R, <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output> as Cmp<UInt<Ur, Br>>>::Output, // R.cmp(UInt<Ur, Br>)
//     <UInt<UI, BI> as Sub<U1>>::Output, // I -= 1
//     Q,
//     UInt<R, <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output>,
//     UInt<Ur, Br>
// >
// {
//     // Remainder: R = R << 1, then R(0) = N(i)
//     // type R = UInt<R, <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output>;
//     type Output = <UInt<Ul, Bl> as PrivateDiv<
//         <UInt<R, <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output> as Cmp<UInt<Ur, Br>>>::Output, // R.cmp(UInt<Ur, Br>)
//     <UInt<UI, BI> as Sub<U1>>::Output, // I -= 1
//     Q,
//     UInt<R, <UInt<Ul, Bl> as BitAt<UInt<UI, BI>>>::Output>,
//     UInt<Ur, Br>
//         >>::Output;
// }

// pub trait PrivateDiv<RvsD, I, Q, R, Rhs> {
//     type Quotient;
//     type Remainder;
// }
