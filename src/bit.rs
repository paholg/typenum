use std::ops::{BitAnd};
use ::{Not, Or, Xor, Same, Cmp, Greater, Less, Equal, SizeOf};
use ::uint::U1;

/// The compile time bit 0
pub struct B0;

/// The compile time bit 1
pub struct B1;

/// The trait for compile time bits; nothing besides B0 and B1 should implement this.
pub trait Bit {
    /// Gives the integer value for this bit.
    fn to_int() -> u8;
    fn to_bool() -> bool;
}

impl Bit for B0 {
    fn to_int() -> u8 { 0 }
    fn to_bool() -> bool { false }
}
impl Bit for B1 {
    fn to_int() -> u8 { 1 }
    fn to_bool() -> bool { true }
}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_bit_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Bit>::to_int(), <Test as Bit>::to_int());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as Same<$Answer>>::Output;
            assert_eq!(<$Answer as Bit>::to_int(), <Test as Bit>::to_int());
        }
        );
}

impl Same<B0> for B0 {
    type Output = B0;
}
impl Same<B1> for B1 {
    type Output = B1;
}

impl SizeOf for B0 {
    type Output = U1;
}
impl SizeOf for B1 {
    type Output = U1;
}

/// Not of 0 (!0 = 1)
impl Not for B0 {
    type Output = B1;
}
/// Not of 1 (!1 = 0)
impl Not for B1 {
    type Output = B0;
}

/// And with 0 ( 0 & B = 0)
impl<Rhs: Bit> BitAnd<Rhs> for B0 {
    type Output = B0;
    fn bitand(self, _: Rhs) -> Self::Output { unreachable!() }
}
/// And with 1 ( 1 & B = B)
impl<Rhs: Bit> BitAnd<Rhs> for B1 {
    type Output = Rhs;
    fn bitand(self, _: Rhs) -> Self::Output { unreachable!() }
}

/// Or with 0 ( 0 | B = B)
impl<Rhs: Bit> Or<Rhs> for B0 {
    type Output = Rhs;
}
/// Or with 1 ( 1 | B = 1)
impl<Rhs: Bit> Or<Rhs> for B1 {
    type Output = B1;
}

/// Xor between 0 and 0 ( 0 ^ 0 = 0)
impl Xor<B0> for B0 {
    type Output = B0;
}
/// Xor between 1 and 0 ( 1 ^ 0 = 1)
impl Xor<B0> for B1 {
    type Output = B1;
}
/// Xor between 0 and 1 ( 0 ^ 1 = 1)
impl Xor<B1> for B0 {
    type Output = B1;
}
/// Xor between 1 and 1 ( 1 ^ 1 = 0)
impl Xor<B1> for B1 {
    type Output = B0;
}

#[test]
fn bit_operations() {
    test_bit_op!(Not B0 = B1);
    test_bit_op!(Not B1 = B0);

    test_bit_op!(B0 BitAnd B0 = B0);
    test_bit_op!(B0 BitAnd B1 = B0);
    test_bit_op!(B1 BitAnd B0 = B0);
    test_bit_op!(B1 BitAnd B1 = B1);

    test_bit_op!(B0 Or B0 = B0);
    test_bit_op!(B0 Or B1 = B1);
    test_bit_op!(B1 Or B0 = B1);
    test_bit_op!(B1 Or B1 = B1);

    test_bit_op!(B0 Xor B0 = B0);
    test_bit_op!(B0 Xor B1 = B1);
    test_bit_op!(B1 Xor B0 = B1);
    test_bit_op!(B1 Xor B1 = B0);
}

impl Cmp<B0> for B0 {
    type Output = Equal;
}

impl Cmp<B1> for B0 {
    type Output = Less;
}

impl Cmp<B0> for B1 {
    type Output = Greater;
}

impl Cmp<B1> for B1 {
    type Output = Equal;
}
