use ::{Not, And, Or, Xor, Same};

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

impl Same<B0> for B0 {
    type Output = B0;
}
impl Same<B1> for B1 {
    type Output = B1;
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
impl<Rhs: Bit> And<Rhs> for B0 {
    type Output = B0;
}
/// And with 1 ( 1 & B = B)
impl<Rhs: Bit> And<Rhs> for B1 {
    type Output = Rhs;
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
    assert_eq!(1, <B0 as Not>::Output::to_int());
    assert_eq!(0, <B1 as Not>::Output::to_int());

    assert_eq!(0, <B0 as And<B0>>::Output::to_int());
    assert_eq!(0, <B0 as And<B1>>::Output::to_int());
    assert_eq!(0, <B1 as And<B0>>::Output::to_int());
    assert_eq!(1, <B1 as And<B1>>::Output::to_int());

    assert_eq!(0, <B0 as Or<B0>>::Output::to_int());
    assert_eq!(1, <B0 as Or<B1>>::Output::to_int());
    assert_eq!(1, <B1 as Or<B0>>::Output::to_int());
    assert_eq!(1, <B1 as Or<B1>>::Output::to_int());

    assert_eq!(0, <B0 as Xor<B0>>::Output::to_int());
    assert_eq!(1, <B0 as Xor<B1>>::Output::to_int());
    assert_eq!(1, <B1 as Xor<B0>>::Output::to_int());
    assert_eq!(0, <B1 as Xor<B1>>::Output::to_int());
}
