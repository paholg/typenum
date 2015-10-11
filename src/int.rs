
use std::marker::PhantomData;

use std::ops::{Neg, Add, Sub};
use ::{NonZero, Same, Cmp, Greater, Equal, Less};
use ::uint::{Unsigned};
use ::__private::{PrivateIntegerAdd};

pub use ::consts::ints::{
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14,
    P15, P16, P17, P18, P19, P20, P21, P22, P23, P24, P25, P26, P27, P28, P29, P30, P31,
    P32, P33, P34, P35, P36, P37, P38, P39, P40, P41, P42, P43, P44, P45, P46, P47, P48,
    P49, P50, P51, P52, P53, P54, P55, P56, P57, P58, P59, P60, P61, P62, P63, P64, P65,
    P66, P67, P68, P69, P70, P71, P72, P73, P74, P75, P76, P77, P78, P79, P80, P81, P82,
    P83, P84, P85, P86, P87, P88, P89, P90, P91, P92, P93, P94, P95, P96, P97, P98, P99,
    P100, P101, P102, P103, P104, P105, P106, P107, P108, P109, P110, P111, P112, P113,
    P114, P115, P116, P117, P118, P119, P120, P121, P122, P123, P124, P125, P126, P127,
    P128, P256, P512, P1024, P2048, P4096, P8192, P10000, P16384, P32768, P65536,

    P131072, P262144, P524288, P1048576, P2097152, P4194304, P8388608, P16777216, P33554432,
    P67108864, P134217728, P268435456, P536870912, P1073741824, P2147483648, P4294967296,
    P8589934592, P17179869184, P34359738368, P68719476736, P137438953472, P274877906944,
    P549755813888, P1099511627776, P2199023255552, P4398046511104, P8796093022208,
    P17592186044416, P35184372088832, P70368744177664, P140737488355328, P281474976710656,
    P562949953421312, P1125899906842624, P2251799813685248, P4503599627370496,
    P9007199254740992, P18014398509481984, P36028797018963968, P72057594037927936,
    P144115188075855872, P288230376151711744, P576460752303423488, P1152921504606846976,
    P2305843009213693952, P4611686018427387904,

    N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14,
    N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31,
    N32, N33, N34, N35, N36, N37, N38, N39, N40, N41, N42, N43, N44, N45, N46, N47, N48,
    N49, N50, N51, N52, N53, N54, N55, N56, N57, N58, N59, N60, N61, N62, N63, N64, N65,
    N66, N67, N68, N69, N70, N71, N72, N73, N74, N75, N76, N77, N78, N79, N80, N81, N82,
    N83, N84, N85, N86, N87, N88, N89, N90, N91, N92, N93, N94, N95, N96, N97, N98, N99,
    N100, N101, N102, N103, N104, N105, N106, N107, N108, N109, N110, N111, N112, N113,
    N114, N115, N116, N117, N118, N119, N120, N121, N122, N123, N124, N125, N126, N127,
    N128, N256, N512, N1024, N2048, N4096, N8192, N10000, N16384, N32768, N65536,

    N131072, N262144, N524288, N1048576, N2097152, N4194304, N8388608, N16777216, N33554432,
    N67108864, N134217728, N268435456, N536870912, N1073741824, N2147483648, N4294967296,
    N8589934592, N17179869184, N34359738368, N68719476736, N137438953472, N274877906944,
    N549755813888, N1099511627776, N2199023255552, N4398046511104, N8796093022208,
    N17592186044416, N35184372088832, N70368744177664, N140737488355328, N281474976710656,
    N562949953421312, N1125899906842624, N2251799813685248, N4503599627370496,
    N9007199254740992, N18014398509481984, N36028797018963968, N72057594037927936,
    N144115188075855872, N288230376151711744, N576460752303423488, N1152921504606846976,
    N2305843009213693952, N4611686018427387904
};



/// Positive integers
pub struct PInt<U: Unsigned + NonZero> {
    _marker: PhantomData<U>
}
/// Negative integers
pub struct NInt<U: Unsigned + NonZero> {
    _marker: PhantomData<U>
}
/// The signed integer 0
pub struct I0;

pub trait Integer {
    fn to_i8() -> i8;
    fn to_i16() -> i16;
    fn to_i32() -> i32;
    fn to_i64() -> i64;
    fn to_isize() -> isize;
}

impl<U: Unsigned + NonZero> NonZero for PInt<U> {}
impl<U: Unsigned + NonZero> NonZero for NInt<U> {}

impl Integer for I0 {
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

impl Same<I0> for I0 {
    type Output = I0;
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

#[test]
fn confirm_ints() {
    assert_eq!(0, I0::to_i64());
    assert_eq!(1, P1::to_i64());
    assert_eq!(2, P2::to_i64());
    assert_eq!(3, P3::to_i64());
    assert_eq!(4, P4::to_i64());
    assert_eq!(5, P5::to_i64());
    assert_eq!(6, P6::to_i64());
    assert_eq!(7, P7::to_i64());
    assert_eq!(8, P8::to_i64());
    assert_eq!(9, P9::to_i64());
    assert_eq!(10, P10::to_i64());
    assert_eq!(11, P11::to_i64());
    assert_eq!(12, P12::to_i64());
    assert_eq!(13, P13::to_i64());
    assert_eq!(14, P14::to_i64());
    assert_eq!(15, P15::to_i64());

    assert_eq!(10000, P10000::to_i64());

    assert_eq!(-1, N1::to_i64());
    assert_eq!(-2, N2::to_i64());
    assert_eq!(-3, N3::to_i64());
    assert_eq!(-4, N4::to_i64());
    assert_eq!(-5, N5::to_i64());
    assert_eq!(-6, N6::to_i64());
    assert_eq!(-7, N7::to_i64());
    assert_eq!(-8, N8::to_i64());
    assert_eq!(-9, N9::to_i64());
    assert_eq!(-10, N10::to_i64());
    assert_eq!(-11, N11::to_i64());
    assert_eq!(-12, N12::to_i64());
    assert_eq!(-13, N13::to_i64());
    assert_eq!(-14, N14::to_i64());
    assert_eq!(-15, N15::to_i64());

    assert_eq!(-10000, N10000::to_i64());
}


// ---------------------------------------------------------------------------------------
// Neg

/// `-I0 = I0`
impl Neg for I0 {
    type Output = I0;
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

#[test]
fn neg_int() {
    test_int_op!(Neg I0 = I0);
    test_int_op!(Neg P1 = N1);
    test_int_op!(Neg N1 = P1);
    test_int_op!(Neg P2305843009213693952 = N2305843009213693952);
    test_int_op!(Neg N2305843009213693952 = P2305843009213693952);
}

// ---------------------------------------------------------------------------------------
// Add

/// `I0 + I = I`
impl<I: Integer> Add<I> for I0 {
    type Output = I;
    fn add(self, _: I) -> Self::Output { unreachable!() }
}

/// `PInt + I0 = PInt`
impl<U: Unsigned + NonZero> Add<I0> for PInt<U> {
    type Output = PInt<U>;
    fn add(self, _: I0) -> Self::Output { unreachable!() }
}

/// `NInt + I0 = NInt`
impl<U: Unsigned + NonZero> Add<I0> for NInt<U> {
    type Output = NInt<U>;
    fn add(self, _: I0) -> Self::Output { unreachable!() }
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
    type Output = I0;
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

#[test]
fn add_ints() {
    test_int_op!(I0 Add I0 = I0);
    test_int_op!(P1 Add I0 = P1);
    test_int_op!(N1 Add I0 = N1);

    test_int_op!(I0 Add P7 = P7);
    test_int_op!(I0 Add N8 = N8);

    test_int_op!(P7 Add P8 = P15);
    test_int_op!(P7 Add N8 = N1);
    test_int_op!(P7 Add N5 = P2);

    test_int_op!(N7 Add N8 = N15);
    test_int_op!(N7 Add P8 = P1);
    test_int_op!(N7 Add P5 = N2);

    test_int_op!(P32768 Add P32768 = P65536);
    test_int_op!(P32768 Add N32768 = I0);
}

// ---------------------------------------------------------------------------------------
// Sub

/// `I0 - I0 = I0`
impl Sub<I0> for I0 {
    type Output = I0;
    fn sub(self, _: I0) -> Self::Output { unreachable!() }
}

/// `I0 - P = N`
impl<U: Unsigned + NonZero> Sub<PInt<U>> for I0 {
    type Output = NInt<U>;
    fn sub(self, _: PInt<U>) -> Self::Output { unreachable!() }
}

/// `I0 - N = P`
impl<U: Unsigned + NonZero> Sub<NInt<U>> for I0 {
    type Output = PInt<U>;
    fn sub(self, _: NInt<U>) -> Self::Output { unreachable!() }
}

/// `PInt - I0 = PInt`
impl<U: Unsigned + NonZero> Sub<I0> for PInt<U> {
    type Output = PInt<U>;
    fn sub(self, _: I0) -> Self::Output { unreachable!() }
}

/// `NInt - I0 = NInt`
impl<U: Unsigned + NonZero> Sub<I0> for NInt<U> {
    type Output = NInt<U>;
    fn sub(self, _: I0) -> Self::Output { unreachable!() }
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

#[test]
fn sub_ints() {
    test_int_op!(I0 Sub I0 = I0);
    test_int_op!(P1 Sub I0 = P1);
    test_int_op!(N1 Sub I0 = N1);

    test_int_op!(I0 Sub P7 = N7);
    test_int_op!(I0 Sub N8 = P8);

    test_int_op!(P7 Sub P8 = N1);
    test_int_op!(P7 Sub N8 = P15);
    test_int_op!(P7 Sub N5 = P12);

    test_int_op!(N7 Sub N8 = P1);
    test_int_op!(N7 Sub P8 = N15);
    test_int_op!(N7 Sub P5 = N12);

    test_int_op!(P32768 Sub P32768 = I0);
    test_int_op!(P32768 Sub N32768 = P65536);
}
