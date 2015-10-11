
use std::marker::PhantomData;

use std::ops::{Neg};
use ::{NonZero, Same};
use ::uint::{Unsigned};

pub use ::consts::ints::{
    I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14,
    I15, I16, I17, I18, I19, I20, I21, I22, I23, I24, I25, I26, I27, I28, I29, I30, I31,
    I32, I33, I34, I35, I36, I37, I38, I39, I40, I41, I42, I43, I44, I45, I46, I47, I48,
    I49, I50, I51, I52, I53, I54, I55, I56, I57, I58, I59, I60, I61, I62, I63, I64, I65,
    I66, I67, I68, I69, I70, I71, I72, I73, I74, I75, I76, I77, I78, I79, I80, I81, I82,
    I83, I84, I85, I86, I87, I88, I89, I90, I91, I92, I93, I94, I95, I96, I97, I98, I99,
    I100, I101, I102, I103, I104, I105, I106, I107, I108, I109, I110, I111, I112, I113,
    I114, I115, I116, I117, I118, I119, I120, I121, I122, I123, I124, I125, I126, I127,
    I128, I256, I512, I1024, I2048, I4096, I8192, I10000, I16384, I32768, I65536,

    I131072, I262144, I524288, I1048576, I2097152, I4194304, I8388608, I16777216, I33554432,
    I67108864, I134217728, I268435456, I536870912, I1073741824, I2147483648, I4294967296,
    I8589934592, I17179869184, I34359738368, I68719476736, I137438953472, I274877906944,
    I549755813888, I1099511627776, I2199023255552, I4398046511104, I8796093022208,
    I17592186044416, I35184372088832, I70368744177664, I140737488355328, I281474976710656,
    I562949953421312, I1125899906842624, I2251799813685248, I4503599627370496,
    I9007199254740992, I18014398509481984, I36028797018963968, I72057594037927936,
    I144115188075855872, I288230376151711744, I576460752303423488, I1152921504606846976,
    I2305843009213693952, I4611686018427387904, I9223372036854775808,

    IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12, IN13, IN14,
    IN15, IN16, IN17, IN18, IN19, IN20, IN21, IN22, IN23, IN24, IN25, IN26, IN27, IN28, IN29, IN30, IN31,
    IN32, IN33, IN34, IN35, IN36, IN37, IN38, IN39, IN40, IN41, IN42, IN43, IN44, IN45, IN46, IN47, IN48,
    IN49, IN50, IN51, IN52, IN53, IN54, IN55, IN56, IN57, IN58, IN59, IN60, IN61, IN62, IN63, IN64, IN65,
    IN66, IN67, IN68, IN69, IN70, IN71, IN72, IN73, IN74, IN75, IN76, IN77, IN78, IN79, IN80, IN81, IN82,
    IN83, IN84, IN85, IN86, IN87, IN88, IN89, IN90, IN91, IN92, IN93, IN94, IN95, IN96, IN97, IN98, IN99,
    IN100, IN101, IN102, IN103, IN104, IN105, IN106, IN107, IN108, IN109, IN110, IN111, IN112, IN113,
    IN114, IN115, IN116, IN117, IN118, IN119, IN120, IN121, IN122, IN123, IN124, IN125, IN126, IN127,
    IN128, IN256, IN512, IN1024, IN2048, IN4096, IN8192, IN10000, IN16384, IN32768, IN65536,

    IN131072, IN262144, IN524288, IN1048576, IN2097152, IN4194304, IN8388608, IN16777216, IN33554432,
    IN67108864, IN134217728, IN268435456, IN536870912, IN1073741824, IN2147483648, IN4294967296,
    IN8589934592, IN17179869184, IN34359738368, IN68719476736, IN137438953472, IN274877906944,
    IN549755813888, IN1099511627776, IN2199023255552, IN4398046511104, IN8796093022208,
    IN17592186044416, IN35184372088832, IN70368744177664, IN140737488355328, IN281474976710656,
    IN562949953421312, IN1125899906842624, IN2251799813685248, IN4503599627370496,
    IN9007199254740992, IN18014398509481984, IN36028797018963968, IN72057594037927936,
    IN144115188075855872, IN288230376151711744, IN576460752303423488, IN1152921504606846976,
    IN2305843009213693952, IN4611686018427387904, IN9223372036854775808
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

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_int_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Integer>::to_u64(), <Test as Integer>::to_u64());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Integer>::to_u64(), <Test as Integer>::to_u64());
        }
        );
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
