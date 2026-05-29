//! Type-level bits.
//!
//! These are rather simple and are used as the building blocks of the
//! other number types in this crate.
//!
//!
//! **Type operators** implemented:
//!
//! - From `core::ops`: `BitAnd`, `BitOr`, `BitXor`, and `Not`.
//! - From `typenum`: `Same` and `Cmp`.

use crate::{
    private::InternalMarker, Cmp, Equal, Greater, Less, NonZero, Ord, PowerOfTwo, Unsigned, Zero,
};
use crate::{IntoUnsigned, U0, U1};
use core::ops::{BitAnd, BitOr, BitXor, Not};

pub use crate::marker_traits::Bit;

/// The type-level bit 0.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
#[cfg_attr(feature = "scale_info", derive(scale_info::TypeInfo))]
pub struct B0;

impl B0 {
    /// Instantiates a singleton representing this bit.
    #[inline]
    pub fn new() -> B0 {
        B0
    }
}

/// The type-level bit 1.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
#[cfg_attr(feature = "scale_info", derive(scale_info::TypeInfo))]
pub struct B1;

impl B1 {
    /// Instantiates a singleton representing this bit.
    #[inline]
    pub fn new() -> B1 {
        B1
    }
}

impl Bit for B0 {
    const U8: u8 = 0;
    const BOOL: bool = false;

    /// Not of 0 (!0 = 1)
    type Not = B1;

    /// And with 0 (0 & B = 0)
    type BitAnd<Rhs: Bit> = B0;

    /// Or with 0 (0 | B = B)
    type BitOr<Rhs: Bit> = Rhs;

    /// Xor with 0 (0 ^ B = B)
    type BitXor<Rhs: Bit> = Rhs;

    /// Min with 0 (min(0, B) = 0)
    type Min<Rhs: Bit> = B0;

    /// Max with 0 (max(0, B) = B)
    type Max<Rhs: Bit> = Rhs;

    /// Comparison with 0 is Less if Rhs is 1, Equal otherwise
    type Cmp<Rhs: Bit> = Rhs::IfOrd<Less, Equal>;
    #[inline]
    fn compare<Rhs: Bit>(_: Rhs) -> Self::Cmp<Rhs> {
        Rhs::if_ord(Less, Equal)
    }

    /// If false then B
    type IfOrd<A: Ord, B: Ord> = B;
    #[inline]
    fn if_ord<A: Ord, B: Ord>(_: A, b: B) -> Self::IfOrd<A, B> {
        b
    }

    /// If false then B
    type IfUnsigned<A: Unsigned, B: Unsigned> = B;
    #[inline]
    fn if_unsigned<A: Unsigned, B: Unsigned>(_: A, b: B) -> Self::IfUnsigned<A, B> {
        b
    }

    type SelectShlUnsigned<S: Unsigned, N: Unsigned> = <S::Double as Unsigned>::Shl<N::Predecessor>;
    #[allow(missing_docs)]
    fn select_shl_unsigned<S: Unsigned, N: Unsigned>(s: S, n: N) -> Self::SelectShlUnsigned<S, N> {
        s.double().shl(n.predecessor())
    }

    #[inline]
    fn new() -> Self {
        Self
    }
    #[inline]
    fn to_u8() -> u8 {
        0
    }
    #[inline]
    fn to_bool() -> bool {
        false
    }
}

impl Bit for B1 {
    const U8: u8 = 1;
    const BOOL: bool = true;

    /// Not of 1 (!1 = 0)
    type Not = B0;

    /// And with 1 (1 & B = B)
    type BitAnd<Rhs: Bit> = Rhs;

    /// Or with 1 (1 | B = 1)
    type BitOr<Rhs: Bit> = B1;

    /// Xor with 1 (1 ^ B = !B)
    type BitXor<Rhs: Bit> = Rhs::Not;

    /// Min with 1 (min(1, B) = B)
    type Min<Rhs: Bit> = Rhs;

    /// Max with 1 (max(1, B) = 1)
    type Max<Rhs: Bit> = B1;

    /// Comparison with 1 is Equal if Rhs is 1, Greater otherwise
    type Cmp<Rhs: Bit> = Rhs::IfOrd<Equal, Greater>;
    #[inline]
    fn compare<Rhs: Bit>(_: Rhs) -> Self::Cmp<Rhs> {
        Rhs::if_ord(Equal, Greater)
    }

    /// If true then A
    type IfOrd<A: Ord, B: Ord> = A;
    #[inline]
    fn if_ord<A: Ord, B: Ord>(a: A, _: B) -> Self::IfOrd<A, B> {
        a
    }

    /// If true then A
    type IfUnsigned<A: Unsigned, B: Unsigned> = A;
    #[inline]
    fn if_unsigned<A: Unsigned, B: Unsigned>(a: A, _: B) -> Self::IfUnsigned<A, B> {
        a
    }

    type SelectShlUnsigned<S: Unsigned, N: Unsigned> = S;
    #[inline]
    fn select_shl_unsigned<S: Unsigned, N: Unsigned>(s: S, _: N) -> Self::SelectShlUnsigned<S, N> {
        s
    }

    #[inline]
    fn new() -> Self {
        Self
    }
    #[inline]
    fn to_u8() -> u8 {
        1
    }
    #[inline]
    fn to_bool() -> bool {
        true
    }
}

impl Zero for B0 {}
impl NonZero for B1 {}
impl PowerOfTwo for B1 {}

macro_rules! delegate_bit_binary_impls {
    (@inner $type:ident, $fn_name:ident, $target:ty) => {
        impl<Rhs: Bit> $type<Rhs> for $target {
            // Delegate the implementation
            type Output = <Self as Bit>::$type<Rhs>;
            #[inline]
            fn $fn_name(self, _: Rhs) -> Self::Output {
                Self::Output::new()
            }
        }
    };
    // `type` is the rust operation trait name, and fn_name the name of the function in this trait
    ($($type:ident, $fn_name:ident,)*) => {
        $(
            // Implementation for B0
            delegate_bit_binary_impls!{@inner $type, $fn_name, B0}
            // Implementation for B1
            delegate_bit_binary_impls!{@inner $type, $fn_name, B1}
        )*
    };
}

delegate_bit_binary_impls! {
    BitAnd, bitand,
    BitOr, bitor,
    BitXor, bitxor,
}

/// Not of 0 (!0 = 1)
impl Not for B0 {
    type Output = <Self as Bit>::Not;
    #[inline]
    fn not(self) -> Self::Output {
        Self::Output::new()
    }
}
/// Not of 1 (!1 = 0)
impl Not for B1 {
    type Output = <Self as Bit>::Not;
    #[inline]
    fn not(self) -> Self::Output {
        Self::Output::new()
    }
}

#[cfg(test)]
mod bit_op_tests {
    use core::ops::{BitAnd, BitOr, BitXor, Not};

    use crate::{B0, B1};

    // macro for testing operation results. Uses `Same` to ensure the types are equal and
    // not just the values they evaluate to.
    macro_rules! test_bit_op {
        ($op:ident $Lhs:ident = $Answer:ident) => {{
            type Test = <<$Lhs as $op>::Output as $crate::Same<$Answer>>::Output;
            assert_eq!(
                <$Answer as $crate::Bit>::to_u8(),
                <Test as $crate::Bit>::to_u8()
            );
        }};
        ($Lhs:ident $op:ident $Rhs:ident = $Answer:ident) => {{
            type Test = <<$Lhs as $op<$Rhs>>::Output as $crate::Same<$Answer>>::Output;
            assert_eq!(
                <$Answer as $crate::Bit>::to_u8(),
                <Test as $crate::Bit>::to_u8()
            );
        }};
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
}

impl<Rhs: Bit> Cmp<Rhs> for B0 {
    type Output = <Self as Bit>::Cmp<Rhs>;
    #[inline]
    fn compare<P: InternalMarker>(&self, _: &Rhs) -> Self::Output {
        Self::Output::new()
    }
}

impl<Rhs: Bit> Cmp<Rhs> for B1 {
    type Output = <Self as Bit>::Cmp<Rhs>;
    #[inline]
    fn compare<P: InternalMarker>(&self, _: &Rhs) -> Self::Output {
        Self::Output::new()
    }
}

use crate::Max;
use crate::Min;
delegate_bit_binary_impls! {
    Min, min,
    Max, max,
}

impl IntoUnsigned for B0 {
    type IntoUnsigned = U0;

    #[inline]
    fn into_unsigned(self) -> Self::IntoUnsigned {
        U0::new()
    }
}

impl IntoUnsigned for B1 {
    type IntoUnsigned = U1;

    #[inline]
    fn into_unsigned(self) -> Self::IntoUnsigned {
        U1::new()
    }
}

#[cfg(test)]
mod bit_creation_tests {
    #[test]
    fn bit_creation() {
        {
            use crate::{B0, B1};
            let _: B0 = B0::new();
            let _: B1 = B1::new();
        }

        {
            use crate::{Bit, B0, B1};

            let _: B0 = <B0 as Bit>::new();
            let _: B1 = <B1 as Bit>::new();
        }
    }
}
