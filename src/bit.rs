/*!

Type-level bits. These are rather simple and are used as the building blocks of the
other number types in this crate.


**Type operators** implemented:

* From std::ops: `BitAnd`, `BitOr`, `BitXor`, and `Not`.
* From typenum: `Same` and `Cmp`.

*/
use std::ops::{BitAnd, BitOr, BitXor, Not};
use {NonZero, Cmp, Greater, Less, Equal};

/// The type-level bit 0.
pub enum B0 {}

/// The type-level bit 1.
pub enum B1 {}

/**
The **marker trait** for compile time bits.

This trait should not be implemented for anything outside this crate.
*/
pub trait Bit {
    fn to_u8() -> u8;
    fn to_bool() -> bool;
}

impl Bit for B0 {
    #[inline] fn to_u8() -> u8 { 0 }
    #[inline] fn to_bool() -> bool { false }
}
impl Bit for B1 {
    #[inline] fn to_u8() -> u8 { 1 }
    #[inline] fn to_bool() -> bool { true }
}

impl NonZero for B1 {}

// macro for testing operation results. Uses `Same` to ensure the types are equal and
// not just the values they evaluate to.
macro_rules! test_bit_op {
    ($op:ident $Lhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op>::Output as ::Same<$Answer>>::Output;
            assert_eq!(<$Answer as Bit>::to_u8(), <Test as Bit>::to_u8());
        }
        );
    ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => (
        {
            type Test = <<$Lhs as $op<$Rhs>>::Output as ::Same<$Answer>>::Output;
            assert_eq!(<$Answer as Bit>::to_u8(), <Test as Bit>::to_u8());
        }
        );
}

/// Not of 0 (!0 = 1)
impl Not for B0 {
    type Output = B1;
    fn not(self) -> Self::Output { unreachable!() }
}
/// Not of 1 (!1 = 0)
impl Not for B1 {
    type Output = B0;
    fn not(self) -> Self::Output { unreachable!() }
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
impl<Rhs: Bit> BitOr<Rhs> for B0 {
    type Output = Rhs;
    fn bitor(self, _: Rhs) -> Self::Output { unreachable!() }
}
/// Or with 1 ( 1 | B = 1)
impl<Rhs: Bit> BitOr<Rhs> for B1 {
    type Output = B1;
    fn bitor(self, _: Rhs) -> Self::Output { unreachable!() }
}

/// Xor between 0 and 0 ( 0 ^ 0 = 0)
impl BitXor<B0> for B0 {
    type Output = B0;
    fn bitxor(self, _: B0) -> Self::Output { unreachable!() }
}
/// Xor between 1 and 0 ( 1 ^ 0 = 1)
impl BitXor<B0> for B1 {
    type Output = B1;
    fn bitxor(self, _: B0) -> Self::Output { unreachable!() }
}
/// Xor between 0 and 1 ( 0 ^ 1 = 1)
impl BitXor<B1> for B0 {
    type Output = B1;
    fn bitxor(self, _: B1) -> Self::Output { unreachable!() }
}
/// Xor between 1 and 1 ( 1 ^ 1 = 0)
impl BitXor<B1> for B1 {
    type Output = B0;
    fn bitxor(self, _: B1) -> Self::Output { unreachable!() }
}

#[test]
fn bit_operations() {
    test_bit_op!(Not B0 = B1);
    test_bit_op!(Not B1 = B0);

    test_bit_op!(B0 BitAnd B0 = B0);
    test_bit_op!(B0 BitAnd B1 = B0);
    test_bit_op!(B1 BitAnd B0 = B0);
    test_bit_op!(B1 BitAnd B1 = B1);

    test_bit_op!(B0 BitOr B0 = B0);
    test_bit_op!(B0 BitOr B1 = B1);
    test_bit_op!(B1 BitOr B0 = B1);
    test_bit_op!(B1 BitOr B1 = B1);

    test_bit_op!(B0 BitXor B0 = B0);
    test_bit_op!(B0 BitXor B1 = B1);
    test_bit_op!(B1 BitXor B0 = B1);
    test_bit_op!(B1 BitXor B1 = B0);
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
