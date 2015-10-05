
use std::marker::PhantomData;

use ::{Same, And, Or, Xor, Add, Sub, Shl, Shr};
use ::bit::{Bit, B0, B1};
use ::private::{Trim, PrivateAnd, PrivateXor, PrivateSub};

pub use ::const_uints::{U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31, U32, U33, U34, U35, U36, U37, U38, U39, U40, U41, U42, U43, U44, U45, U46, U47, U48, U49, U50, U51, U52, U53, U54, U55, U56, U57, U58, U59, U60, U61, U62, U63, U64, U65, U66, U67, U68, U69, U70, U71, U72, U73, U74, U75, U76, U77, U78, U79, U80, U81, U82, U83, U84, U85, U86, U87, U88, U89, U90, U91, U92, U93, U94, U95, U96, U97, U98, U99, U100, U101, U102, U103, U104, U105, U106, U107, U108, U109, U110, U111, U112, U113, U114, U115, U116, U117, U118, U119, U120, U121, U122, U123, U124, U125, U126, U127, U128, U256, U512, U1024, U2048, U4096, U8192, U16384, U32768, U65536};

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
}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_uint_op {
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Unsigned>::to_int(), <Test as Unsigned>::to_int());
        }
        );
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
    test_uint_op!(U0 Add B1 = U1);
    test_uint_op!(U1 Add B1 = U2);
    test_uint_op!(U7 Add B1 = U8);
    test_uint_op!(U7 Add B0 = U7);
    test_uint_op!(U16 Add B0 = U16);
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
    test_uint_op!(U0 Add U0 = U0);
    test_uint_op!(U1 Add U0 = U1);
    test_uint_op!(U7 Add U2 = U9);
    test_uint_op!(U31 Add U31 = U62);
    test_uint_op!(U32 Add U31 = U63);
    test_uint_op!(U31 Add U32 = U63);
}

// Subtracting bits from unsigned integers -----------------------------------------------

/// Subtracting the 0 bit from any `Unsigned`: `U - B0 = U`
impl<U: Unsigned> Sub<B0> for U {
    type Output = U;
}
/// Subtracting the 1 bit from a `UInt` with final bit 1: `UInt<U, B1> - B1 = UInt<U, B0>`
impl<U: Unsigned, B: Bit> Sub<B1> for UInt<UInt<U, B>, B1> {
    type Output = UInt<UInt<U, B>, B0>;
}

// Subtracting the last 1 bit from a value
impl Sub<B1> for UInt<UTerm, B1> {
    type Output = UTerm;
}

/// Subtracting the 1 bit from a `UInt` with final bit 0: `UInt<U, B0> - B1 = UInt<U - B1, B1>`
impl<U: Unsigned> Sub<B1> for UInt<U, B0> where U:Sub<B1>, <U as Sub<B1>>::Output: Unsigned {
    type Output = UInt<<U as Sub<B1>>::Output, B1>;
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

/// A **type operation** used to determine when to borrow for subtraction. Notice that
/// this is a non-commutative operation, as we only borrow when we have 0 - 1.
///
/// Table:
/// ```
///  0 0 | 0
///  0 1 | 1
///  1 0 | 0
///  1 1 | 0
/// ```

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



/// Subtracting unsigned integers:
impl<Ul: Unsigned, Ur: Unsigned> Sub<Ur> for Ul
    where Ul: Sub<Ur>
{
    type Output = <<Ul as PrivateSub<Ur>>::Output as Trim>::Output;
}

/// Subtracting `UTerm` from anything: `U - UTerm = UTerm`
impl<U: Unsigned> PrivateSub<UTerm> for U {
    type Output = U;
}

/// `UInt<Ul, Bl> - UInt<Ur, Br> = UInt<(Ul - Bl Borrow Br) - Ur, Bl ^ Br>`
/// where `Borrow` is a **type operation** that only returns `B1` when
/// we need to borrow; `Bl = 0` and `Br = 1`. The rest of the time it returns `B0`.
impl<Bl: Bit, Ul: Unsigned, Br: Bit, Ur: Unsigned> PrivateSub<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Bl: Xor<Br> + Borrow<Br>,
          Ul: Sub<<Bl as Borrow<Br>>::Output>,
          <Ul as Sub<<Bl as Borrow<Br>>::Output>>::Output: PrivateSub<Ur>,
          <<Ul as Sub<<Bl as Borrow<Br>>::Output>>::Output as PrivateSub<Ur>>::Output: Unsigned,
          <Bl as Xor<Br>>::Output: Bit
{
    type Output = UInt<
        <<Ul as Sub<<Bl as Borrow<Br>>::Output>>::Output as PrivateSub<Ur>>::Output,
        <Bl as Xor<Br>>::Output>;
}

#[test]
fn sub_uints() {
    // Uncomment for error:
    // type TestN1 = <U0 as Sub<U1>>::Output;
    // assert_eq!(-1, <TestN1 as Unsigned>::to_int());

    // type Test00 = <<U0 as Sub<U0>>::Output as Same<U0>>::Output;
    // type Test10 = <<U1 as Sub<U0>>::Output as Same<U1>>::Output;
    // type Test11 = <<U1 as Sub<U1>>::Output as Same<U0>>::Output;
    // type Test20 = <<U2 as Sub<U0>>::Output as Same<U2>>::Output;
    // type Test21 = <<U2 as Sub<U1>>::Output as Same<U1>>::Output;
    // type Test22 = <<U2 as Sub<U2>>::Output as Same<U0>>::Output;

    // assert_eq!(0, <Test00 as Unsigned>::to_int());
    // assert_eq!(1, <Test10 as Unsigned>::to_int());
    // assert_eq!(0, <Test11 as Unsigned>::to_int());
    // assert_eq!(2, <Test20 as Unsigned>::to_int());
    // assert_eq!(1, <Test21 as Unsigned>::to_int());
    // assert_eq!(0, <Test22 as Unsigned>::to_int());


    // type Test6432 = <U64 as Sub<U32>>::Output;
    // assert_eq!(32, <Test6432 as Unsigned>::to_int());

    // type Test3131 = <U31 as Sub<U31>>::Output;
    // assert_eq!(0, <Test3131 as Unsigned>::to_int());

    // type Test3231 = <U32 as Sub<U31>>::Output;
    // assert_eq!(1, <Test3231 as Unsigned>::to_int());
}

/// Anding `UTerm` with anything: `UTerm & X = UTerm`
impl<U: Unsigned> PrivateAnd<U> for UTerm {
    type Output = UTerm;
}
/// Anding `UTerm` with anything: `X & UTerm = UTerm`
impl<B: Bit, U: Unsigned> PrivateAnd<UTerm> for UInt<U, B> {
    type Output = UTerm;
}

/// Anding unsigned integers: `UInt<Ul, Bl> & UInt<Ur, Br> = UInt<Ul & Ur, Bl & Br>`
impl<Bl: Bit, Ul: Unsigned, Br: Bit, Ur: Unsigned> PrivateAnd<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Ul: PrivateAnd<Ur>, Bl: And<Br>, <Bl as And<Br>>::Output: Bit,
        <Ul as PrivateAnd<Ur>>::Output: Unsigned
{
    type Output = UInt<
        <Ul as PrivateAnd<Ur>>::Output,
        <Bl as And<Br>>::Output>;
}

impl<Ul: Unsigned, Ur: Unsigned> And<Ur> for Ul
    where Ul: PrivateAnd<Ur>,
          <Ul as PrivateAnd<Ur>>::Output: Trim
{
    type Output = <<Ul as PrivateAnd<Ur>>::Output as Trim>::Output;
}

#[test]
fn and_uints() {
    test_uint_op!(U0 And U0 = U0);
    test_uint_op!(U1 And U0 = U0);
    test_uint_op!(U0 And U1 = U0);
    test_uint_op!(U1 And U1 = U1);

    test_uint_op!(U2 And U9 = U0);
    test_uint_op!(U9 And U2 = U0);
    test_uint_op!(U127 And U128 = U0);
    test_uint_op!(U3 And U7 = U3);
    test_uint_op!(U15 And U15 = U15);

    test_uint_op!(U120 And U105 = U104);
}

/// Oring `UTerm` with anything: `UTerm | X = X`
impl<U: Unsigned> Or<U> for UTerm {
    type Output = U;
}
/// Oring `UTerm` with anything: `X | UTerm = X`
impl<B: Bit, U: Unsigned> Or<UTerm> for UInt<U, B> {
    type Output = Self;
}

/// Oring unsigned integers: `UInt<Ul, Bl> | UInt<Ur, Br> = UInt<Ul | Ur, Bl | Br>`
impl<Bl: Bit, Ul: Unsigned, Br: Bit, Ur: Unsigned> Or<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Ul: Or<Ur>, Bl: Or<Br>, <Bl as Or<Br>>::Output: Bit,
        <Ul as Or<Ur>>::Output: Unsigned
{
    type Output = UInt<
        <Ul as Or<Ur>>::Output,
        <Bl as Or<Br>>::Output>;
}

#[test]
fn or_uints() {
    test_uint_op!(U0 Or U0 = U0);
    test_uint_op!(U1 Or U0 = U1);
    test_uint_op!(U0 Or U1 = U1);
    test_uint_op!(U1 Or U1 = U1);


    test_uint_op!(U2 Or U9 = U11);
    test_uint_op!(U3 Or U7 = U7);

    test_uint_op!(U15 Or U15 = U15);
}

/// Exclusive-Oring `UTerm` with anything: `UTerm ^ X = X`
impl<U: Unsigned> Xor<U> for UTerm {
    type Output = U;
}
/// Exclusive-Oring `UTerm` with anything: `X ^ UTerm = X`
impl<B: Bit, U: Unsigned> Xor<UTerm> for UInt<U, B> {
    type Output = Self;
}

/// Exclusive-Oring unsigned integers: `UInt<Ul, Bl> ^ UInt<Ur, Br> = UInt<Ul ^ Ur, Bl ^ Br>`
impl<Bl: Bit, Ul: Unsigned, Br: Bit, Ur: Unsigned> Xor<UInt<Ur, Br>> for UInt<Ul, Bl>
    where Ul: Xor<Ur>, Bl: Xor<Br>, <Bl as Xor<Br>>::Output: Bit,
        <Ul as Xor<Ur>>::Output: Unsigned
{
    type Output = UInt<
        <Ul as Xor<Ur>>::Output,
        <Bl as Xor<Br>>::Output>;
}

#[test]
fn xor_uints() {
    // fixme: Need to trim for Xor
    test_uint_op!(U0 Xor U0 = U0);
    test_uint_op!(U1 Xor U0 = U1);
    test_uint_op!(U0 Xor U1 = U1);
    //test_uint_op!(U1 Xor U1 = U0);

    test_uint_op!(U2 Xor U9 = U11);
    test_uint_op!(U3 Xor U7 = U4);

    //test_uint_op!(U15 Xor U15 = U15);
}

/// Shifting left `UTerm` by anything: `UTerm << X = UTerm`
impl<U: Unsigned> Shl<U> for UTerm {
    type Output = UTerm;
}

/// Shifting left `UInt` by `UTerm`: `X << UTerm = X`
impl<U: Unsigned, B: Bit> Shl<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
}

/// Shifting left by a zero bit: `X << B0 = X`
impl<U: Unsigned, B: Bit> Shl<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
}

/// Shifting left by a one bit: `X << B1 = UInt<X, B0>`
impl<U: Unsigned, B: Bit> Shl<B1> for UInt<U, B> {
    type Output = UInt<UInt<U, B>, B0>;
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
}

#[test]
fn shl_tests() {
    test_uint_op!(U0 Shl U0 = U0);
    test_uint_op!(U1 Shl U0 = U1);
    test_uint_op!(U0 Shl U1 = U0);
    test_uint_op!(U1 Shl U1 = U2);

    test_uint_op!(U2 Shl U9 = U1024);
    test_uint_op!(U7 Shl U3 = U56);

    test_uint_op!(U1 Shl U15 = U32768);
}

/// Shifting right a `UTerm` by anything: `UTerm >> X = UTerm`
impl<U: Unsigned> Shr<U> for UTerm {
    type Output = UTerm;
}

/// Shifting right `UInt` by `UTerm`: `X >> UTerm = X`
impl<U: Unsigned, B: Bit> Shr<UTerm> for UInt<U, B> {
    type Output = UInt<U, B>;
}

/// Shifting right by a zero bit: `X >> B0 = X`
impl<U: Unsigned, B: Bit> Shr<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
}

/// Shifting right by a one bit: `UInt<X, B> >> B1 = X`
impl<U: Unsigned, B: Bit> Shr<B1> for UInt<U, B> {
    type Output = U;
}

/// Shifting right `UInt` by `UInt`: `UInt(U, B) >> Y` = `U >> (Y - 1)`
impl<U: Unsigned, B: Bit, Ur: Unsigned, Br: Bit> Shr<UInt<Ur, Br>> for UInt<U, B>
where UInt<Ur, Br> : Sub<B1>,
    U : Shr<<UInt<Ur, Br> as Sub<B1>>::Output>
{
    type Output = <U as Shr<<UInt<Ur, Br> as Sub<B1>>::Output>>::Output;
}

#[test]
fn shr_tests() {
    test_uint_op!(U0 Shr U0 = U0);
    test_uint_op!(U1 Shr U0 = U1);
    test_uint_op!(U0 Shr U1 = U0);
    test_uint_op!(U1 Shr U1 = U0);

    test_uint_op!(U9 Shr U2 = U2);
    test_uint_op!(U7 Shr U3 = U0);

    test_uint_op!(U65536 Shr U15 = U2);
}


