//! Type-level unsigned integers.
//!
//!
//! **Type operators** implemented:
//!
//! From `::core::ops`: `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`, `Add`, `Sub`,
//!                 `Mul`, `Div`, and `Rem`.
//! From `typenum`: `Same`, `Cmp`, and `Pow`.
//!
//! Rather than directly using the structs defined in this module, it is recommended that
//! you import and use the relevant aliases from the [consts](../consts/index.html) module.
//!
//! # Example
//! ```rust
//! use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};
//! use typenum::{Unsigned, U1, U2, U3, U4};
//!
//! assert_eq!(<U3 as BitAnd<U2>>::Output::to_u32(), 2);
//! assert_eq!(<U3 as BitOr<U4>>::Output::to_u32(), 7);
//! assert_eq!(<U3 as BitXor<U2>>::Output::to_u32(), 1);
//! assert_eq!(<U3 as Shl<U1>>::Output::to_u32(), 6);
//! assert_eq!(<U3 as Shr<U1>>::Output::to_u32(), 1);
//! assert_eq!(<U3 as Add<U2>>::Output::to_u32(), 5);
//! assert_eq!(<U3 as Sub<U2>>::Output::to_u32(), 1);
//! assert_eq!(<U3 as Mul<U2>>::Output::to_u32(), 6);
//! assert_eq!(<U3 as Div<U2>>::Output::to_u32(), 1);
//! assert_eq!(<U3 as Rem<U2>>::Output::to_u32(), 1);
//! ```

use crate::{
    bit::{Bit, B0, B1},
    consts::{U0, U1},
    private::{
        BitDiff, BitDiffOut, Internal, InternalMarker, PrivateLogarithm2, PrivatePow,
        PrivatePowOut, PrivateSquareRoot, PrivateSub, PrivateSubOut, Trim, TrimOut,
    },
    Add1, Cmp, Double, Equal, Gcd, Gcf, GrEq, Greater, IntoUnsigned, IsGreaterOrEqual, Len, Length,
    Less, Log2, Logarithm2, Maximum, Minimum, NonZero, Ord, Pow, Prod, Shleft, Sqrt, Square,
    SquareRoot, Sub1, Sum, ToInt, TypeAdd, TypeBitAnd, TypeBitOr, TypeBitXor, TypeMul, TypeShl,
    TypeShr, Zero,
};
use core::ops::{Add, BitAnd, BitOr, BitXor, Mul, Shl, Shr, Sub};

pub use crate::marker_traits::{PowerOfTwo, Unsigned};

/// The terminating type for `UInt`; it always comes after the most significant
/// bit. `UTerm` by itself represents zero, which is aliased to `U0`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
#[cfg_attr(feature = "scale_info", derive(scale_info::TypeInfo))]
pub struct UTerm;

impl UTerm {
    /// Instantiates a singleton representing this unsigned integer.
    #[inline]
    pub fn new() -> UTerm {
        UTerm
    }
}

impl Unsigned for UTerm {
    const U8: u8 = 0;
    const U16: u16 = 0;
    const U32: u32 = 0;
    const U64: u64 = 0;
    #[cfg(feature = "i128")]
    const U128: u128 = 0;
    const USIZE: usize = 0;

    const I8: i8 = 0;
    const I16: i16 = 0;
    const I32: i32 = 0;
    const I64: i64 = 0;
    #[cfg(feature = "i128")]
    const I128: i128 = 0;
    const ISIZE: isize = 0;

    #[inline]
    fn to_u8() -> u8 {
        0
    }
    #[inline]
    fn to_u16() -> u16 {
        0
    }
    #[inline]
    fn to_u32() -> u32 {
        0
    }
    #[inline]
    fn to_u64() -> u64 {
        0
    }
    #[cfg(feature = "i128")]
    #[inline]
    fn to_u128() -> u128 {
        0
    }
    #[inline]
    fn to_usize() -> usize {
        0
    }

    #[inline]
    fn to_i8() -> i8 {
        0
    }
    #[inline]
    fn to_i16() -> i16 {
        0
    }
    #[inline]
    fn to_i32() -> i32 {
        0
    }
    #[inline]
    fn to_i64() -> i64 {
        0
    }
    #[cfg(feature = "i128")]
    #[inline]
    fn to_i128() -> i128 {
        0
    }
    #[inline]
    fn to_isize() -> isize {
        0
    }

    type GetMSB = UTerm;
    #[inline]
    fn get_msb(self) -> Self::GetMSB {
        UTerm
    }

    type GetLSB = B0;
    #[inline]
    fn get_lsb(self) -> Self::GetLSB {
        B0
    }

    /// `0 + 1 = 1`
    type Successor = UInt<UTerm, B1>;
    #[inline]
    fn successor(self) -> Self::Successor {
        U1::new()
    }

    /// `0 - 1`,  SubstractOverflowError
    type Predecessor = UInt<UTerm, B1>;
    #[inline]
    fn predecessor(self) -> Self::Predecessor {
        U1::new()
    }

    /// `0` is already trimmed.
    type Trimmed = UTerm;
    #[inline]
    fn trimmed(self) -> Self::Trimmed {
        UTerm
    }

    type IsZero = B1;
    #[inline]
    fn is_zero(self) -> Self::IsZero {
        B1
    }

    /// `min(0, Rhs) = 0`
    type Min<Rhs: Unsigned> = UTerm;
    #[inline]
    fn min<Rhs: Unsigned>(self, _: Rhs) -> Self::Min<Rhs> {
        UTerm
    }

    /// `max(0, Rhs) = Rhs`
    type Max<Rhs: Unsigned> = Rhs;
    #[inline]
    fn max<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Max<Rhs> {
        rhs
    }

    /// `0 & Rhs = 0`
    type BitAnd<Rhs: Unsigned> = UTerm;
    #[inline]
    fn bitand<Rhs: Unsigned>(self, _: Rhs) -> Self::BitAnd<Rhs> {
        UTerm
    }

    /// `0 | Rhs = Rhs`
    type BitOr<Rhs: Unsigned> = Rhs;
    #[inline]
    fn bitor<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitOr<Rhs> {
        rhs
    }

    /// `0 ^ Rhs = Rhs`
    type BitXor<Rhs: Unsigned> = Rhs;
    #[inline]
    fn bitxor<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitXor<Rhs> {
        rhs
    }

    /// `0 + Rhs = Rhs`
    type Add<Rhs: Unsigned> = Rhs;
    #[inline]
    fn add<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Add<Rhs> {
        rhs
    }

    /// `0 + Rhs + Carry = Rhs + Carry`
    type AddCarry<Rhs: Unsigned, Carry: Bit> = Carry::IfUnsigned<Rhs::Successor, Rhs>;
    #[inline]
    fn add_carry<Rhs: Unsigned, Carry: Bit>(self, rhs: Rhs) -> Self::AddCarry<Rhs, Carry> {
        Carry::if_unsigned(rhs.successor(), rhs)
    }

    /// `0 * Rhs = 0`
    type Mul<Rhs: Unsigned> = UTerm;
    #[allow(missing_docs)]
    fn mul<Rhs: Unsigned>(self, _: Rhs) -> Self::Mul<Rhs> {
        UTerm
    }

    /// `0 >> Rhs = 0`
    type Shr<Rhs: Unsigned> = UTerm;
    #[inline]
    fn shr<Rhs: Unsigned>(self, _: Rhs) -> Self::Shr<Rhs> {
        self
    }

    /// `0 << Rhs = 0`
    type Shl<Rhs: Unsigned> = UTerm;
    #[inline]
    fn shl<Rhs: Unsigned>(self, _: Rhs) -> Self::Shl<Rhs> {
        self
    }

    type Double = UTerm;
    #[inline]
    fn double(self) -> Self::Double {
        self
    }

    type Cmp<Rhs: Unsigned> = <Rhs::IsZero as Bit>::IfOrd<Equal, Less>;
    #[inline]
    fn compare<Rhs: Unsigned>(self, _: Rhs) -> Self::Cmp<Rhs> {
        Rhs::IsZero::if_ord(Equal, Less)
    }
}

/// `UInt` is defined recursively, where `B` is the least significant bit and `U` is the rest
/// of the number. Conceptually, `U` should be bound by the trait `Unsigned` and `B` should
/// be bound by the trait `Bit`, but enforcing these bounds causes linear instead of
/// logrithmic scaling in some places, so they are left off for now. They may be enforced in
/// future.
///
/// In order to keep numbers unique, leading zeros are not allowed, so `UInt<UTerm, B0>` is
/// forbidden.
///
/// # Example
/// ```rust
/// use typenum::{UInt, UTerm, B0, B1};
///
/// # #[allow(dead_code)]
/// type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
#[cfg_attr(feature = "scale_info", derive(scale_info::TypeInfo))]
pub struct UInt<U, B> {
    /// The more significant bits of `Self`.
    pub(crate) msb: U,
    /// The least significant bit of `Self`.
    pub(crate) lsb: B,
}

impl<U: Unsigned, B: Bit> UInt<U, B> {
    /// Instantiates a singleton representing this unsigned integer.
    #[inline]
    pub fn new() -> UInt<U, B> {
        UInt::default()
    }
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {
    const U8: u8 = B::U8 | U::U8 << 1;
    const U16: u16 = B::U8 as u16 | U::U16 << 1;
    const U32: u32 = B::U8 as u32 | U::U32 << 1;
    const U64: u64 = B::U8 as u64 | U::U64 << 1;
    #[cfg(feature = "i128")]
    const U128: u128 = B::U8 as u128 | U::U128 << 1;
    const USIZE: usize = B::U8 as usize | U::USIZE << 1;

    const I8: i8 = B::U8 as i8 | U::I8 << 1;
    const I16: i16 = B::U8 as i16 | U::I16 << 1;
    const I32: i32 = B::U8 as i32 | U::I32 << 1;
    const I64: i64 = B::U8 as i64 | U::I64 << 1;
    #[cfg(feature = "i128")]
    const I128: i128 = B::U8 as i128 | U::I128 << 1;
    const ISIZE: isize = B::U8 as isize | U::ISIZE << 1;

    #[inline]
    fn to_u8() -> u8 {
        B::to_u8() | U::to_u8() << 1
    }
    #[inline]
    fn to_u16() -> u16 {
        u16::from(B::to_u8()) | U::to_u16() << 1
    }
    #[inline]
    fn to_u32() -> u32 {
        u32::from(B::to_u8()) | U::to_u32() << 1
    }
    #[inline]
    fn to_u64() -> u64 {
        u64::from(B::to_u8()) | U::to_u64() << 1
    }
    #[cfg(feature = "i128")]
    #[inline]
    fn to_u128() -> u128 {
        u128::from(B::to_u8()) | U::to_u128() << 1
    }
    #[inline]
    fn to_usize() -> usize {
        usize::from(B::to_u8()) | U::to_usize() << 1
    }

    #[inline]
    fn to_i8() -> i8 {
        B::to_u8() as i8 | U::to_i8() << 1
    }
    #[inline]
    fn to_i16() -> i16 {
        i16::from(B::to_u8()) | U::to_i16() << 1
    }
    #[inline]
    fn to_i32() -> i32 {
        i32::from(B::to_u8()) | U::to_i32() << 1
    }
    #[inline]
    fn to_i64() -> i64 {
        i64::from(B::to_u8()) | U::to_i64() << 1
    }
    #[cfg(feature = "i128")]
    #[inline]
    fn to_i128() -> i128 {
        i128::from(B::to_u8()) | U::to_i128() << 1
    }
    #[inline]
    fn to_isize() -> isize {
        B::to_u8() as isize | U::to_isize() << 1
    }

    type GetMSB = U;
    #[inline]
    fn get_msb(self) -> Self::GetMSB {
        self.msb
    }

    type GetLSB = B;
    #[inline]
    fn get_lsb(self) -> Self::GetLSB {
        self.lsb
    }

    /// `Self + 1`
    type Successor = B::IfUnsigned<UInt<U::Successor, B0>, UInt<U, B1>>;
    #[inline]
    fn successor(self) -> Self::Successor {
        B::if_unsigned(
            UInt {
                msb: self.get_msb().successor(),
                lsb: B0,
            },
            UInt {
                msb: self.get_msb(),
                lsb: B1,
            },
        )
    }

    /// `Self - 1`
    type Predecessor = <B::IfUnsigned<UInt<U, B0>, UInt<U::Predecessor, B1>> as Unsigned>::Trimmed; // Trim the output because of potential leadings 0.
    #[inline]
    fn predecessor(self) -> Self::Predecessor {
        B::if_unsigned(
            UInt {
                msb: self.get_msb(),
                lsb: B0,
            },
            UInt {
                msb: self.get_msb().predecessor(),
                lsb: B1,
            },
        )
        .trimmed()
    }

    /// Remove leading 0 of `Self`.
    type Trimmed = <<U::Trimmed as Unsigned>::IsZero as Bit>::IfUnsigned<
        B::IfUnsigned<UInt<UTerm, B>, UTerm>,
        UInt<U::Trimmed, B>,
    >;
    #[inline]
    fn trimmed(self) -> Self::Trimmed {
        todo!()
    }

    // Self can't be zero if trimmed.
    type IsZero = B0;
    #[inline]
    fn is_zero(self) -> Self::IsZero {
        B0
    }

    type Min<Rhs: Unsigned> = <<Self::Cmp<Rhs> as Ord>::IsLess as Bit>::IfUnsigned<Self, Rhs>;
    #[inline]
    fn min<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Min<Rhs> {
        <Self::Cmp<Rhs> as Ord>::IsLess::if_unsigned(self, rhs)
    }

    type Max<Rhs: Unsigned> = <<Self::Cmp<Rhs> as Ord>::IsLess as Bit>::IfUnsigned<Rhs, Self>;
    #[inline]
    fn max<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Max<Rhs> {
        <Self::Cmp<Rhs> as Ord>::IsLess::if_unsigned(rhs, self)
    }

    /// `UInt<U, B> & UInt<Ur, Br> = UInt<U & Ur, B & Br>`
    type BitAnd<Rhs: Unsigned> =
        <UInt<U::BitAnd<Rhs::GetMSB>, B::BitAnd<Rhs::GetLSB>> as Unsigned>::Trimmed; // Trim the output because of potential leadings 0.
    #[inline]
    fn bitand<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitAnd<Rhs> {
        UInt {
            msb: self.get_msb().bitand(rhs.get_msb()),
            lsb: B::BitAnd::<Rhs::GetLSB>::new(),
        }
        .trimmed()
    }

    /// `UInt<U, B> | UInt<Ur, Br> = UInt<U | Ur, B | Br>`
    type BitOr<Rhs: Unsigned> = UInt<U::BitOr<Rhs::GetMSB>, B::BitOr<Rhs::GetLSB>>;
    #[inline]
    fn bitor<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitOr<Rhs> {
        UInt {
            msb: self.get_msb().bitor(rhs.get_msb()),
            lsb: B::BitOr::<Rhs::GetLSB>::new(),
        }
    }

    /// `UInt<U, B> ^ UInt<Ur, Br> = UInt<U ^ Ur, B ^ Br>`
    type BitXor<Rhs: Unsigned> =
        <UInt<U::BitXor<Rhs::GetMSB>, B::BitXor<Rhs::GetLSB>> as Unsigned>::Trimmed; // Trim the output because of potential leadings 0.
    #[inline]
    fn bitxor<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitXor<Rhs> {
        UInt {
            msb: self.get_msb().bitxor(rhs.get_msb()),
            lsb: B::BitXor::<Rhs::GetLSB>::new(),
        }
        .trimmed()
    }

    /// `UInt<U, B> + UInt<Ur, Br> = UInt<U + Ur + (B & Br), B ^ Br>`
    type Add<Rhs: Unsigned> =
        UInt<U::AddCarry<Rhs::GetMSB, B::BitAnd<Rhs::GetLSB>>, B::BitXor<Rhs::GetLSB>>;
    #[inline]
    fn add<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Add<Rhs> {
        UInt {
            msb: U::add_carry(self.get_msb(), rhs.get_msb()),
            lsb: B::BitXor::<Rhs::GetLSB>::new(),
        }
    }
    /// `Self + Rhs + Carry = Self + (if Carry { Rhs + 1 } else { Rhs })`
    type AddCarry<Rhs: Unsigned, Carry: Bit> = Self::Add<Carry::IfUnsigned<Rhs::Successor, Rhs>>;
    #[inline]
    fn add_carry<Rhs: Unsigned, Carry: Bit>(self, rhs: Rhs) -> Self::AddCarry<Rhs, Carry> {
        Unsigned::add(self, Carry::if_unsigned(rhs.successor(), rhs))
    }

    /// `UInt<U, B> * Rhs = if B { Rhs } else { 0 } + U * (Rhs << 1)`
    type Mul<Rhs: Unsigned> = <B::IfUnsigned<Rhs, UTerm> as Unsigned>::Add<U::Mul<Rhs::Double>>;
    #[allow(missing_docs)]
    fn mul<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Mul<Rhs> {
        Unsigned::add(B::if_unsigned(rhs, UTerm), self.get_msb().mul(rhs.double()))
    }

    /// `UInt<U, B> >> Rhs = if Rhs == 0 { Self } else { U >> (Rhs - 1) }`
    type Shr<Rhs: Unsigned> = <Rhs::IsZero as Bit>::IfUnsigned<Self, U::Shr<Rhs::Predecessor>>;
    #[inline]
    fn shr<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Shr<Rhs> {
        Rhs::IsZero::if_unsigned(self, self.get_msb().shr(rhs.predecessor()))
    }

    /// `Self << Rhs = if Rhs == 0 { Self } else { UInt<Self, B0> << (Rhs - 1) }`
    /// The logic is implemented on the `Bit` trait to avoid overflow when the compiler checks types.
    type Shl<Rhs: Unsigned> = <Rhs::IsZero as Bit>::SelectShlUnsigned<Self, Rhs>;
    #[inline]
    fn shl<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Shl<Rhs> {
        <Rhs::IsZero as Bit>::select_shl_unsigned(self, rhs)
    }

    /// `Self << 1 = UInt<Self, B0>`
    type Double = UInt<Self, B0>;
    #[inline]
    fn double(self) -> Self::Double {
        UInt { msb: self, lsb: B0 }
    }

    /// Compare msb, then if they are equal compare lsb.
    type Cmp<Rhs: Unsigned> = <U::Cmp<Rhs::GetMSB> as Ord>::Then<B::Cmp<Rhs::GetLSB>>;
    #[allow(missing_docs)]
    fn compare<Rhs: Unsigned>(self, _: Rhs) -> Self::Cmp<Rhs> {
        Self::Cmp::<Rhs>::new()
    }
}

impl<U: Unsigned, B: Bit> NonZero for UInt<U, B> {}
impl Zero for UTerm {}

impl PowerOfTwo for UInt<UTerm, B1> {}
impl<U: Unsigned + PowerOfTwo> PowerOfTwo for UInt<U, B0> {}

// ---------------------------------------------------------------------------------------
// Getting length of unsigned integers, which is defined as the number of bits before `UTerm`

/// Length of `UTerm` by itself is 0
impl Len for UTerm {
    type Output = U0;
    #[inline]
    fn len(&self) -> Self::Output {
        UTerm
    }
}

/// Length of a bit is 1
impl<U: Unsigned, B: Bit> Len for UInt<U, B>
where
    U: Len,
{
    type Output = <Length<U> as Unsigned>::Successor;
    #[inline]
    fn len(&self) -> Self::Output {
        self.msb.len().successor()
    }
}

// ---------------------------------------------------------------------------------------
// Formatting as binary

impl core::fmt::Binary for UTerm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0")
    }
}

impl core::fmt::Binary for UInt<UTerm, B0> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0")
    }
}

impl core::fmt::Binary for UInt<UTerm, B1> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "1")
    }
}

impl<U: Unsigned, B: Bit> core::fmt::Binary for UInt<UInt<U, B>, B0>
where
    UInt<U, B>: core::fmt::Binary,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:b}0", UInt::<U, B>::new())
    }
}

impl<U: Unsigned, B: Bit> core::fmt::Binary for UInt<UInt<U, B>, B1>
where
    UInt<U, B>: core::fmt::Binary,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:b}1", UInt::<U, B>::new())
    }
}

#[cfg(test)]
mod fmt_tests {
    use super::*;
    use crate::consts::*;
    use core::fmt::Write;

    struct LimitedString {
        len: usize,
        buffer: [u8; 64],
    }

    impl LimitedString {
        fn new() -> Self {
            Self {
                len: 0,
                buffer: [0u8; 64],
            }
        }

        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.buffer[..self.len]).unwrap()
        }
    }

    impl core::fmt::Write for LimitedString {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.buffer[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
            self.len += s.len();
            Ok(())
        }
    }

    fn assert_binary_fmt<U: Unsigned + core::fmt::Binary>(expected: &str) {
        let mut s = LimitedString::new();
        write!(&mut s, "{:b}", U::default()).unwrap();
        assert_eq!(s.as_str(), expected);
    }

    #[test]
    fn binary() {
        assert_binary_fmt::<U0>("0");
        assert_binary_fmt::<U1>("1");
        assert_binary_fmt::<U2>("10");
        assert_binary_fmt::<U3>("11");
        assert_binary_fmt::<U4>("100");
        assert_binary_fmt::<U5>("101");
        assert_binary_fmt::<U6>("110");
        assert_binary_fmt::<U7>("111");
        assert_binary_fmt::<U8>("1000");
        assert_binary_fmt::<U9>("1001");
        assert_binary_fmt::<U10>("1010");

        assert_binary_fmt::<U2147483648>("10000000000000000000000000000000");
    }
}

macro_rules! delegate_unsigned_binary_impls {
    // `type` is the rust operation trait name, `custom_type` is the custom trait name associated with the operation,
    // and fn_name the name of the function in this trait
    ($($type:ident, $custom_type:ident, $fn_name:ident,)*) => {
        $(
            // Implementation for UTerm
            impl<Rhs: IntoUnsigned> $type<Rhs> for UTerm {
                // Delegate the implementation
                type Output = <Self as Unsigned>::$type<Rhs::IntoUnsigned>;
                #[inline]
                fn $fn_name(self, rhs: Rhs) -> Self::Output {
                    <Self as Unsigned>::$fn_name(self, rhs.into_unsigned())
                }
            }
            // Implementation for UInt<U, B>
            impl<U: Unsigned, B: Bit, Rhs: IntoUnsigned> $type<Rhs> for UInt<U, B> {
                // Delegate the implementation
                type Output = <Self as Unsigned>::$type<Rhs::IntoUnsigned>;
                #[inline]
                fn $fn_name(self, rhs: Rhs) -> Self::Output {
                    <Self as Unsigned>::$fn_name(self, rhs.into_unsigned())
                }
            }
            // Implementation of custom trait
            impl<U: Unsigned, Rhs: IntoUnsigned> $custom_type<Rhs> for U {
                // Delegate the implementation
                type Output = <Self as Unsigned>::$type<Rhs::IntoUnsigned>;
                #[inline]
                fn $fn_name(self, rhs: Rhs) -> Self::Output {
                    <Self as Unsigned>::$fn_name(self, rhs.into_unsigned())
                }
            }
        )*
    };
    ($($custom_type:ident, $fn_name:ident,)*) => {
        $(
            // Implementation of custom trait
            impl<U: Unsigned, Rhs: IntoUnsigned> $custom_type<Rhs> for U {
                // Delegate the implementation
                type Output = <Self as Unsigned>::$custom_type<Rhs::IntoUnsigned>;
                #[inline]
                fn $fn_name(self, rhs: Rhs) -> Self::Output {
                    <Self as Unsigned>::$fn_name(self, rhs.into_unsigned())
                }
            }
        )*
    };
}

// ---------------------------------------------------------------------------------------
// Adding bits to unsigned integers

delegate_unsigned_binary_impls! {
    Add, TypeAdd, add,
}

// ---------------------------------------------------------------------------------------
// Subtracting bits from unsigned integers

/// `UTerm - B0 = Term`
impl Sub<B0> for UTerm {
    type Output = UTerm;
    #[inline]
    fn sub(self, _: B0) -> Self::Output {
        UTerm
    }
}

/// `UInt - B0 = UInt`
impl<U: Unsigned, B: Bit> Sub<B0> for UInt<U, B> {
    type Output = UInt<U, B>;
    #[inline]
    fn sub(self, _: B0) -> Self::Output {
        UInt::new()
    }
}

/// `UInt<U, B1> - B1 = UInt<U, B0>`
impl<U: Unsigned, B: Bit> Sub<B1> for UInt<UInt<U, B>, B1> {
    type Output = UInt<UInt<U, B>, B0>;
    #[inline]
    fn sub(self, _: B1) -> Self::Output {
        UInt::new()
    }
}

/// `UInt<UTerm, B1> - B1 = UTerm`
impl Sub<B1> for UInt<UTerm, B1> {
    type Output = UTerm;
    #[inline]
    fn sub(self, _: B1) -> Self::Output {
        UTerm
    }
}

/// `UInt<U, B0> - B1 = UInt<U - B1, B1>`
impl<U: Unsigned> Sub<B1> for UInt<U, B0>
where
    U: Sub<B1>,
    Sub1<U>: Unsigned,
{
    type Output = UInt<Sub1<U>, B1>;
    #[inline]
    fn sub(self, _: B1) -> Self::Output {
        UInt::new()
    }
}

// ---------------------------------------------------------------------------------------
// Subtracting unsigned integers

/// `UTerm - UTerm = UTerm`
impl Sub<UTerm> for UTerm {
    type Output = UTerm;
    #[inline]
    fn sub(self, _: UTerm) -> Self::Output {
        UTerm
    }
}

/// Subtracting unsigned integers. We just do our `PrivateSub` and then `Trim` the output.
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned> Sub<Ur> for UInt<Ul, Bl>
where
    UInt<Ul, Bl>: PrivateSub<Ur>,
    PrivateSubOut<UInt<Ul, Bl>, Ur>: Trim,
{
    type Output = TrimOut<PrivateSubOut<UInt<Ul, Bl>, Ur>>;
    #[inline]
    fn sub(self, rhs: Ur) -> Self::Output {
        self.private_sub(rhs).trim()
    }
}

/// `U - UTerm = U`
impl<U: Unsigned> PrivateSub<UTerm> for U {
    type Output = U;

    #[inline]
    fn private_sub(self, _: UTerm) -> Self::Output {
        self
    }
}

/// `UInt<Ul, B0> - UInt<Ur, B0> = UInt<Ul - Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B0>> for UInt<Ul, B0>
where
    Ul: PrivateSub<Ur>,
{
    type Output = UInt<PrivateSubOut<Ul, Ur>, B0>;

    #[inline]
    fn private_sub(self, rhs: UInt<Ur, B0>) -> Self::Output {
        UInt {
            msb: self.msb.private_sub(rhs.msb),
            lsb: B0,
        }
    }
}

/// `UInt<Ul, B0> - UInt<Ur, B1> = UInt<(Ul - Ur) - B1, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B1>> for UInt<Ul, B0>
where
    Ul: PrivateSub<Ur>,
    PrivateSubOut<Ul, Ur>: Sub<B1>,
{
    type Output = UInt<Sub1<PrivateSubOut<Ul, Ur>>, B1>;

    #[inline]
    fn private_sub(self, rhs: UInt<Ur, B1>) -> Self::Output {
        UInt {
            msb: self.msb.private_sub(rhs.msb) - B1,
            lsb: B1,
        }
    }
}

/// `UInt<Ul, B1> - UInt<Ur, B0> = UInt<Ul - Ur, B1>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B0>> for UInt<Ul, B1>
where
    Ul: PrivateSub<Ur>,
{
    type Output = UInt<PrivateSubOut<Ul, Ur>, B1>;

    #[inline]
    fn private_sub(self, rhs: UInt<Ur, B0>) -> Self::Output {
        UInt {
            msb: self.msb.private_sub(rhs.msb),
            lsb: B1,
        }
    }
}

/// `UInt<Ul, B1> - UInt<Ur, B1> = UInt<Ul - Ur, B0>`
impl<Ul: Unsigned, Ur: Unsigned> PrivateSub<UInt<Ur, B1>> for UInt<Ul, B1>
where
    Ul: PrivateSub<Ur>,
{
    type Output = UInt<PrivateSubOut<Ul, Ur>, B0>;

    #[inline]
    fn private_sub(self, rhs: UInt<Ur, B1>) -> Self::Output {
        UInt {
            msb: self.msb.private_sub(rhs.msb),
            lsb: B0,
        }
    }
}

// ---------------------------------------------------------------------------------------
// And unsigned integers

delegate_unsigned_binary_impls! {
    BitAnd, TypeBitAnd, bitand,
}

// ---------------------------------------------------------------------------------------
// Or unsigned integers

delegate_unsigned_binary_impls! {
    BitOr, TypeBitOr, bitor,
}

// ---------------------------------------------------------------------------------------
// Xor unsigned integers

delegate_unsigned_binary_impls! {
    BitXor, TypeBitXor, bitxor,
}

// ---------------------------------------------------------------------------------------
// Shl unsigned integers

delegate_unsigned_binary_impls! {
    Shl, TypeShl, shl,
}

// ---------------------------------------------------------------------------------------
// Shr unsigned integers

delegate_unsigned_binary_impls! {
    Shr, TypeShr, shr,
}

// ---------------------------------------------------------------------------------------
// Multiply unsigned integers

delegate_unsigned_binary_impls! {
    Mul, TypeMul, mul,
}

// ---------------------------------------------------------------------------------------
// Compare unsigned integers

impl<U: Unsigned, Rhs: IntoUnsigned> Cmp<Rhs> for U {
    // Delegate the implementation
    type Output = <Self as Unsigned>::Cmp<Rhs::IntoUnsigned>;
    #[inline]
    fn compare<Im: InternalMarker>(&self, _: &Rhs) -> Self::Output {
        <Self as Unsigned>::Cmp::<Rhs::IntoUnsigned>::new()
    }
}

// ---------------------------------------------------------------------------------------
// Getting difference in number of bits

impl<Ul, Bl, Ur, Br> BitDiff<UInt<Ur, Br>> for UInt<Ul, Bl>
where
    Ul: Unsigned,
    Bl: Bit,
    Ur: Unsigned,
    Br: Bit,
    Ul: BitDiff<Ur>,
{
    type Output = BitDiffOut<Ul, Ur>;
}

impl<Ul> BitDiff<UTerm> for Ul
where
    Ul: Unsigned + Len,
{
    type Output = Length<Ul>;
}

// ---------------------------------------------------------------------------------------
// Shifting one number until it's the size of another
use crate::private::ShiftDiff;
impl<Ul: Unsigned, Ur: Unsigned> ShiftDiff<Ur> for Ul
where
    Ur: BitDiff<Ul>,
    Ul: Shl<BitDiffOut<Ur, Ul>>,
{
    type Output = Shleft<Ul, BitDiffOut<Ur, Ul>>;
}

// ---------------------------------------------------------------------------------------
// Powers of unsigned integers

/// X^N
impl<X: Unsigned, N: Unsigned> Pow<N> for X
where
    X: PrivatePow<U1, N>,
{
    type Output = PrivatePowOut<X, U1, N>;
    #[inline]
    fn powi(self, n: N) -> Self::Output {
        self.private_pow(U1::new(), n)
    }
}

impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U0> for X {
    type Output = Y;

    #[inline]
    fn private_pow(self, y: Y, _: U0) -> Self::Output {
        y
    }
}

impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U1> for X
where
    X: Mul<Y>,
{
    type Output = Prod<X, Y>;

    #[inline]
    fn private_pow(self, y: Y, _: U1) -> Self::Output {
        self * y
    }
}

/// N is even
impl<Y: Unsigned, U: Unsigned, B: Bit, X: Unsigned> PrivatePow<Y, UInt<UInt<U, B>, B0>> for X
where
    X: Mul,
    Square<X>: PrivatePow<Y, UInt<U, B>>,
{
    type Output = PrivatePowOut<Square<X>, Y, UInt<U, B>>;

    #[inline]
    fn private_pow(self, y: Y, n: UInt<UInt<U, B>, B0>) -> Self::Output {
        (self * self).private_pow(y, n.msb)
    }
}

/// N is odd
impl<Y: Unsigned, U: Unsigned, B: Bit, X: Unsigned> PrivatePow<Y, UInt<UInt<U, B>, B1>> for X
where
    X: Mul + Mul<Y>,
    Square<X>: PrivatePow<Prod<X, Y>, UInt<U, B>>,
{
    type Output = PrivatePowOut<Square<X>, Prod<X, Y>, UInt<U, B>>;

    #[inline]
    fn private_pow(self, y: Y, n: UInt<UInt<U, B>, B1>) -> Self::Output {
        (self * self).private_pow(self * y, n.msb)
    }
}

//------------------------------------------
// Greatest Common Divisor

/// The even number 2*N
#[allow(unused)] // Silence spurious warning on older versions of rust
type Even<N> = UInt<N, B0>;

/// The odd number 2*N + 1
type Odd<N> = UInt<N, B1>;

/// gcd(0, 0) = 0
impl Gcd<U0> for U0 {
    type Output = U0;
}

/// gcd(x, 0) = x
impl<X> Gcd<U0> for X
where
    X: Unsigned + NonZero,
{
    type Output = X;
}

/// gcd(0, y) = y
impl<Y> Gcd<Y> for U0
where
    Y: Unsigned + NonZero,
{
    type Output = Y;
}

/// gcd(x, y) = 2*gcd(x/2, y/2) if both x and y even
impl<Xp, Yp> Gcd<Even<Yp>> for Even<Xp>
where
    Xp: Gcd<Yp>,
    Even<Xp>: NonZero,
    Even<Yp>: NonZero,
{
    type Output = UInt<Gcf<Xp, Yp>, B0>;
}

/// gcd(x, y) = gcd(x, y/2) if x odd and y even
impl<Xp, Yp> Gcd<Even<Yp>> for Odd<Xp>
where
    Odd<Xp>: Gcd<Yp>,
    Even<Yp>: NonZero,
{
    type Output = Gcf<Odd<Xp>, Yp>;
}

/// gcd(x, y) = gcd(x/2, y) if x even and y odd
impl<Xp, Yp> Gcd<Odd<Yp>> for Even<Xp>
where
    Xp: Gcd<Odd<Yp>>,
    Even<Xp>: NonZero,
{
    type Output = Gcf<Xp, Odd<Yp>>;
}

/// gcd(x, y) = gcd([max(x, y) - min(x, y)], min(x, y)) if both x and y odd
///
/// This will immediately invoke the case for x even and y odd because the difference of two odd
/// numbers is an even number.
impl<Xp, Yp> Gcd<Odd<Yp>> for Odd<Xp>
where
    Odd<Xp>: Max<Odd<Yp>> + Min<Odd<Yp>>,
    Odd<Yp>: Max<Odd<Xp>> + Min<Odd<Xp>>,
    Maximum<Odd<Xp>, Odd<Yp>>: Sub<Minimum<Odd<Xp>, Odd<Yp>>>,
    Diff<Maximum<Odd<Xp>, Odd<Yp>>, Minimum<Odd<Xp>, Odd<Yp>>>: Gcd<Minimum<Odd<Xp>, Odd<Yp>>>,
{
    type Output =
        Gcf<Diff<Maximum<Odd<Xp>, Odd<Yp>>, Minimum<Odd<Xp>, Odd<Yp>>>, Minimum<Odd<Xp>, Odd<Yp>>>;
}

#[cfg(test)]
mod gcd_tests {
    use super::*;
    use crate::consts::*;

    macro_rules! gcd_test {
        (
            $( $a:ident, $b:ident => $c:ident ),* $(,)*
        ) => {
            $(
                assert_eq!(<Gcf<$a, $b> as Unsigned>::to_usize(), $c::to_usize());
                assert_eq!(<Gcf<$b, $a> as Unsigned>::to_usize(), $c::to_usize());
             )*
        }
    }

    #[test]
    fn gcd() {
        gcd_test! {
            U0,   U0    => U0,
            U0,   U42   => U42,
            U12,  U8    => U4,
            U13,  U1013 => U1,  // Two primes
            U9,   U26   => U1,  // Not prime but coprime
            U143, U273  => U13,
            U117, U273  => U39,
        }
    }
}

// -----------------------------------------
// GetBit

#[allow(missing_docs)]
pub trait GetBit<I> {
    #[allow(missing_docs)]
    type Output;

    #[doc(hidden)]
    fn get_bit<IM: InternalMarker>(&self, _: &I) -> Self::Output;
}

#[allow(missing_docs)]
pub type GetBitOut<N, I> = <N as GetBit<I>>::Output;

// Base case
impl<Un, Bn> GetBit<U0> for UInt<Un, Bn>
where
    Bn: Copy,
{
    type Output = Bn;

    #[inline]
    fn get_bit<IM: InternalMarker>(&self, _: &U0) -> Self::Output {
        self.lsb
    }
}

// Recursion case
impl<Un, Bn, Ui, Bi> GetBit<UInt<Ui, Bi>> for UInt<Un, Bn>
where
    UInt<Ui, Bi>: Copy + Sub<B1>,
    Un: GetBit<Sub1<UInt<Ui, Bi>>>,
{
    type Output = GetBitOut<Un, Sub1<UInt<Ui, Bi>>>;

    #[inline]
    fn get_bit<IM: InternalMarker>(&self, i: &UInt<Ui, Bi>) -> Self::Output {
        self.msb.get_bit::<Internal>(&(*i - B1))
    }
}

// Ran out of bits
impl<I> GetBit<I> for UTerm {
    type Output = B0;

    #[inline]
    fn get_bit<IM: InternalMarker>(&self, _: &I) -> Self::Output {
        B0
    }
}

#[test]
fn test_get_bit() {
    use crate::consts::*;
    use crate::Same;
    type T1 = <GetBitOut<U2, U0> as Same<B0>>::Output;
    type T2 = <GetBitOut<U2, U1> as Same<B1>>::Output;
    type T3 = <GetBitOut<U2, U2> as Same<B0>>::Output;

    <T1 as Bit>::to_bool();
    <T2 as Bit>::to_bool();
    <T3 as Bit>::to_bool();
}

// -----------------------------------------
// SetBit

/// A **type operator** that, when implemented for unsigned integer `N`, sets the bit at position
/// `I` to `B`.
pub trait SetBit<I, B> {
    #[allow(missing_docs)]
    type Output;

    #[doc(hidden)]
    fn set_bit<IM: InternalMarker>(self, _: I, _: B) -> Self::Output;
}
/// Alias for the result of calling `SetBit`: `SetBitOut<N, I, B> = <N as SetBit<I, B>>::Output`.
pub type SetBitOut<N, I, B> = <N as SetBit<I, B>>::Output;

use crate::private::{PrivateSetBit, PrivateSetBitOut};

// Call private one then trim it
impl<N, I, B> SetBit<I, B> for N
where
    N: PrivateSetBit<I, B>,
    PrivateSetBitOut<N, I, B>: Trim,
{
    type Output = TrimOut<PrivateSetBitOut<N, I, B>>;

    #[inline]
    fn set_bit<IM: InternalMarker>(self, i: I, b: B) -> Self::Output {
        self.private_set_bit(i, b).trim()
    }
}

// Base case
impl<Un, Bn, B> PrivateSetBit<U0, B> for UInt<Un, Bn> {
    type Output = UInt<Un, B>;

    #[inline]
    fn private_set_bit(self, _: U0, b: B) -> Self::Output {
        UInt {
            msb: self.msb,
            lsb: b,
        }
    }
}

// Recursion case
impl<Un, Bn, Ui, Bi, B> PrivateSetBit<UInt<Ui, Bi>, B> for UInt<Un, Bn>
where
    UInt<Ui, Bi>: Sub<B1>,
    Un: PrivateSetBit<Sub1<UInt<Ui, Bi>>, B>,
{
    type Output = UInt<PrivateSetBitOut<Un, Sub1<UInt<Ui, Bi>>, B>, Bn>;

    #[inline]
    fn private_set_bit(self, i: UInt<Ui, Bi>, b: B) -> Self::Output {
        UInt {
            msb: self.msb.private_set_bit(i - B1, b),
            lsb: self.lsb,
        }
    }
}

// Ran out of bits, setting B0
impl<I> PrivateSetBit<I, B0> for UTerm {
    type Output = UTerm;

    #[inline]
    fn private_set_bit(self, _: I, _: B0) -> Self::Output {
        UTerm
    }
}

// Ran out of bits, setting B1
impl<I> PrivateSetBit<I, B1> for UTerm
where
    U1: Shl<I>,
{
    type Output = Shleft<U1, I>;

    #[inline]
    fn private_set_bit(self, i: I, _: B1) -> Self::Output {
        <U1 as Shl<I>>::shl(U1::new(), i)
    }
}

#[test]
fn test_set_bit() {
    use crate::consts::*;
    use crate::Same;
    type T1 = <SetBitOut<U2, U0, B0> as Same<U2>>::Output;
    type T2 = <SetBitOut<U2, U0, B1> as Same<U3>>::Output;
    type T3 = <SetBitOut<U2, U1, B0> as Same<U0>>::Output;
    type T4 = <SetBitOut<U2, U1, B1> as Same<U2>>::Output;
    type T5 = <SetBitOut<U2, U2, B0> as Same<U2>>::Output;
    type T6 = <SetBitOut<U2, U2, B1> as Same<U6>>::Output;
    type T7 = <SetBitOut<U2, U3, B0> as Same<U2>>::Output;
    type T8 = <SetBitOut<U2, U3, B1> as Same<U10>>::Output;
    type T9 = <SetBitOut<U2, U4, B0> as Same<U2>>::Output;
    type T10 = <SetBitOut<U2, U4, B1> as Same<U18>>::Output;

    type T11 = <SetBitOut<U3, U0, B0> as Same<U2>>::Output;

    <T1 as Unsigned>::to_u32();
    <T2 as Unsigned>::to_u32();
    <T3 as Unsigned>::to_u32();
    <T4 as Unsigned>::to_u32();
    <T5 as Unsigned>::to_u32();
    <T6 as Unsigned>::to_u32();
    <T7 as Unsigned>::to_u32();
    <T8 as Unsigned>::to_u32();
    <T9 as Unsigned>::to_u32();
    <T10 as Unsigned>::to_u32();
    <T11 as Unsigned>::to_u32();
}

// -----------------------------------------

// Division algorithm:
// We have N / D:
// let Q = 0, R = 0
// NBits = len(N)
// for I in NBits-1..0:
//   R <<=1
//   R[0] = N[i]
//   let C = R.cmp(D)
//   if C == Equal or Greater:
//     R -= D
//     Q[i] = 1

#[cfg(test)]
mod div_tests {
    use crate::Unsigned;

    use super::SetBitOut;

    macro_rules! test_div {
        ($a:ident / $b:ident = $c:ident) => {{
            type R = Quot<$a, $b>;
            assert_eq!(<R as Unsigned>::to_usize(), $c::to_usize());
        }};
    }
    #[test]
    fn test_div() {
        use crate::consts::*;
        use crate::{Quot, Same};

        test_div!(U0 / U1 = U0);
        test_div!(U1 / U1 = U1);
        test_div!(U2 / U1 = U2);
        test_div!(U3 / U1 = U3);
        test_div!(U4 / U1 = U4);

        test_div!(U0 / U2 = U0);
        test_div!(U1 / U2 = U0);
        test_div!(U2 / U2 = U1);
        test_div!(U3 / U2 = U1);
        test_div!(U4 / U2 = U2);
        test_div!(U6 / U2 = U3);
        test_div!(U7 / U2 = U3);

        type T = <SetBitOut<U0, U1, B1> as Same<U2>>::Output;
        <T as Unsigned>::to_u32();
    }
}
// -----------------------------------------
// Div
use core::ops::Div;

// 0 // N
impl<Ur: Unsigned, Br: Bit> Div<UInt<Ur, Br>> for UTerm {
    type Output = UTerm;
    #[inline]
    fn div(self, _: UInt<Ur, Br>) -> Self::Output {
        UTerm
    }
}

// M // N
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit> Div<UInt<Ur, Br>> for UInt<Ul, Bl>
where
    UInt<Ul, Bl>: Len,
    Length<UInt<Ul, Bl>>: Sub<B1>,
    (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
{
    type Output = PrivateDivQuot<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
    #[inline]
    fn div(self, rhs: UInt<Ur, Br>) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        ().private_div_quotient(self, rhs, U0::new(), U0::new(), self.len() - B1)
    }
}

// -----------------------------------------
// Rem
use core::ops::Rem;

// 0 % N
impl<Ur: Unsigned, Br: Bit> Rem<UInt<Ur, Br>> for UTerm {
    type Output = UTerm;
    #[inline]
    fn rem(self, _: UInt<Ur, Br>) -> Self::Output {
        UTerm
    }
}

// M % N
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit> Rem<UInt<Ur, Br>> for UInt<Ul, Bl>
where
    UInt<Ul, Bl>: Len,
    Length<UInt<Ul, Bl>>: Sub<B1>,
    (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
{
    type Output = PrivateDivRem<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
    #[inline]
    fn rem(self, rhs: UInt<Ur, Br>) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        ().private_div_remainder(self, rhs, UTerm, UTerm, self.len() - B1)
    }
}

// -----------------------------------------
// PrivateDiv
use crate::private::{PrivateDiv, PrivateDivQuot, PrivateDivRem};

use crate::Compare;
// R == 0: We set R = UInt<UTerm, N[i]>, then call out to PrivateDivIf for the if statement
impl<N, D, Q, I> PrivateDiv<N, D, Q, U0, I> for ()
where
    N: GetBit<I>,
    UInt<UTerm, GetBitOut<N, I>>: Trim,
    TrimOut<UInt<UTerm, GetBitOut<N, I>>>: Cmp<D>,
    (): PrivateDivIf<
        N,
        D,
        Q,
        TrimOut<UInt<UTerm, GetBitOut<N, I>>>,
        I,
        Compare<TrimOut<UInt<UTerm, GetBitOut<N, I>>>, D>,
    >,
{
    type Quotient = PrivateDivIfQuot<
        N,
        D,
        Q,
        TrimOut<UInt<UTerm, GetBitOut<N, I>>>,
        I,
        Compare<TrimOut<UInt<UTerm, GetBitOut<N, I>>>, D>,
    >;
    type Remainder = PrivateDivIfRem<
        N,
        D,
        Q,
        TrimOut<UInt<UTerm, GetBitOut<N, I>>>,
        I,
        Compare<TrimOut<UInt<UTerm, GetBitOut<N, I>>>, D>,
    >;

    #[inline]
    fn private_div_quotient(self, n: N, d: D, q: Q, _: U0, i: I) -> Self::Quotient
where {
        let r = (UInt {
            msb: UTerm,
            lsb: n.get_bit::<Internal>(&i),
        })
        .trim();
        let r_cmp_d = r.compare::<Internal>(&d);
        ().private_div_if_quotient(n, d, q, r, i, r_cmp_d)
    }

    #[inline]
    fn private_div_remainder(self, n: N, d: D, q: Q, _: U0, i: I) -> Self::Remainder {
        let r = (UInt {
            msb: UTerm,
            lsb: n.get_bit::<Internal>(&i),
        })
        .trim();
        let r_cmp_d = r.compare::<Internal>(&d);
        ().private_div_if_remainder(n, d, q, r, i, r_cmp_d)
    }
}

// R > 0: We perform R <<= 1 and R[0] = N[i], then call out to PrivateDivIf for the if statement
impl<N, D, Q, Ur, Br, I> PrivateDiv<N, D, Q, UInt<Ur, Br>, I> for ()
where
    N: GetBit<I>,
    UInt<UInt<Ur, Br>, GetBitOut<N, I>>: Cmp<D>,
    (): PrivateDivIf<
        N,
        D,
        Q,
        UInt<UInt<Ur, Br>, GetBitOut<N, I>>,
        I,
        Compare<UInt<UInt<Ur, Br>, GetBitOut<N, I>>, D>,
    >,
{
    type Quotient = PrivateDivIfQuot<
        N,
        D,
        Q,
        UInt<UInt<Ur, Br>, GetBitOut<N, I>>,
        I,
        Compare<UInt<UInt<Ur, Br>, GetBitOut<N, I>>, D>,
    >;
    type Remainder = PrivateDivIfRem<
        N,
        D,
        Q,
        UInt<UInt<Ur, Br>, GetBitOut<N, I>>,
        I,
        Compare<UInt<UInt<Ur, Br>, GetBitOut<N, I>>, D>,
    >;

    #[inline]
    fn private_div_quotient(self, n: N, d: D, q: Q, r: UInt<Ur, Br>, i: I) -> Self::Quotient {
        let r = UInt {
            msb: r,
            lsb: n.get_bit::<Internal>(&i),
        };
        let r_cmp_d = r.compare::<Internal>(&d);
        ().private_div_if_quotient(n, d, q, r, i, r_cmp_d)
    }

    #[inline]
    fn private_div_remainder(self, n: N, d: D, q: Q, r: UInt<Ur, Br>, i: I) -> Self::Remainder {
        let r = UInt {
            msb: r,
            lsb: n.get_bit::<Internal>(&i),
        };
        let r_cmp_d = r.compare::<Internal>(&d);
        ().private_div_if_remainder(n, d, q, r, i, r_cmp_d)
    }
}

// -----------------------------------------
// PrivateDivIf

use crate::private::{PrivateDivIf, PrivateDivIfQuot, PrivateDivIfRem};

// R < D, I > 0, we do nothing and recurse
impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Less> for ()
where
    UInt<Ui, Bi>: Sub<B1>,
    (): PrivateDiv<N, D, Q, R, Sub1<UInt<Ui, Bi>>>,
{
    type Quotient = PrivateDivQuot<N, D, Q, R, Sub1<UInt<Ui, Bi>>>;
    type Remainder = PrivateDivRem<N, D, Q, R, Sub1<UInt<Ui, Bi>>>;

    #[inline]
    fn private_div_if_quotient(
        self,
        n: N,
        d: D,
        q: Q,
        r: R,
        i: UInt<Ui, Bi>,
        _: Less,
    ) -> Self::Quotient
where {
        ().private_div_quotient(n, d, q, r, i - B1)
    }

    #[inline]
    fn private_div_if_remainder(
        self,
        n: N,
        d: D,
        q: Q,
        r: R,
        i: UInt<Ui, Bi>,
        _: Less,
    ) -> Self::Remainder
where {
        ().private_div_remainder(n, d, q, r, i - B1)
    }
}

// R == D, I > 0, we set R = 0, Q[I] = 1 and recurse
impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Equal> for ()
where
    UInt<Ui, Bi>: Copy + Sub<B1>,
    Q: SetBit<UInt<Ui, Bi>, B1>,
    (): PrivateDiv<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>,
{
    type Quotient = PrivateDivQuot<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>;
    type Remainder = PrivateDivRem<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>;

    #[inline]
    fn private_div_if_quotient(
        self,
        n: N,
        d: D,
        q: Q,
        _: R,
        i: UInt<Ui, Bi>,
        _: Equal,
    ) -> Self::Quotient
where {
        ().private_div_quotient(n, d, q.set_bit::<Internal>(i, B1), U0::new(), i - B1)
    }

    #[inline]
    fn private_div_if_remainder(
        self,
        n: N,
        d: D,
        q: Q,
        _: R,
        i: UInt<Ui, Bi>,
        _: Equal,
    ) -> Self::Remainder
where {
        ().private_div_remainder(n, d, q.set_bit::<Internal>(i, B1), U0::new(), i - B1)
    }
}

use crate::Diff;
// R > D, I > 0, we set R -= D, Q[I] = 1 and recurse
impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Greater> for ()
where
    D: Copy,
    UInt<Ui, Bi>: Copy + Sub<B1>,
    R: Sub<D>,
    Q: SetBit<UInt<Ui, Bi>, B1>,
    (): PrivateDiv<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, Diff<R, D>, Sub1<UInt<Ui, Bi>>>,
{
    type Quotient =
        PrivateDivQuot<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, Diff<R, D>, Sub1<UInt<Ui, Bi>>>;
    type Remainder =
        PrivateDivRem<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, Diff<R, D>, Sub1<UInt<Ui, Bi>>>;

    #[inline]
    fn private_div_if_quotient(
        self,
        n: N,
        d: D,
        q: Q,
        r: R,
        i: UInt<Ui, Bi>,
        _: Greater,
    ) -> Self::Quotient
where {
        ().private_div_quotient(n, d, q.set_bit::<Internal>(i, B1), r - d, i - B1)
    }

    #[inline]
    fn private_div_if_remainder(
        self,
        n: N,
        d: D,
        q: Q,
        r: R,
        i: UInt<Ui, Bi>,
        _: Greater,
    ) -> Self::Remainder
where {
        ().private_div_remainder(n, d, q.set_bit::<Internal>(i, B1), r - d, i - B1)
    }
}

// R < D, I == 0: we do nothing, and return
impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, U0, Less> for () {
    type Quotient = Q;
    type Remainder = R;

    #[inline]
    fn private_div_if_quotient(self, _: N, _: D, q: Q, _: R, _: U0, _: Less) -> Self::Quotient {
        q
    }

    #[inline]
    fn private_div_if_remainder(self, _: N, _: D, _: Q, r: R, _: U0, _: Less) -> Self::Remainder {
        r
    }
}

// R == D, I == 0: we set R = 0, Q[I] = 1, and return
impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, U0, Equal> for ()
where
    Q: SetBit<U0, B1>,
{
    type Quotient = SetBitOut<Q, U0, B1>;
    type Remainder = U0;

    #[inline]
    fn private_div_if_quotient(self, _: N, _: D, q: Q, _: R, i: U0, _: Equal) -> Self::Quotient {
        q.set_bit::<Internal>(i, B1)
    }

    #[inline]
    fn private_div_if_remainder(self, _: N, _: D, _: Q, _: R, i: U0, _: Equal) -> Self::Remainder {
        i
    }
}

// R > D, I == 0: We set R -= D, Q[I] = 1, and return
impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, U0, Greater> for ()
where
    R: Sub<D>,
    Q: SetBit<U0, B1>,
{
    type Quotient = SetBitOut<Q, U0, B1>;
    type Remainder = Diff<R, D>;

    #[inline]
    fn private_div_if_quotient(self, _: N, _: D, q: Q, _: R, i: U0, _: Greater) -> Self::Quotient {
        q.set_bit::<Internal>(i, B1)
    }

    #[inline]
    fn private_div_if_remainder(
        self,
        _: N,
        d: D,
        _: Q,
        r: R,
        _: U0,
        _: Greater,
    ) -> Self::Remainder {
        r - d
    }
}

// -----------------------------------------
// PartialDiv
use crate::{PartialDiv, Quot};
impl<Ur: Unsigned, Br: Bit> PartialDiv<UInt<Ur, Br>> for UTerm {
    type Output = UTerm;
    #[inline]
    fn partial_div(self, _: UInt<Ur, Br>) -> Self::Output {
        UTerm
    }
}

// M / N
impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit> PartialDiv<UInt<Ur, Br>> for UInt<Ul, Bl>
where
    UInt<Ul, Bl>: Div<UInt<Ur, Br>> + Rem<UInt<Ur, Br>, Output = U0>,
{
    type Output = Quot<UInt<Ul, Bl>, UInt<Ur, Br>>;
    #[inline]
    fn partial_div(self, rhs: UInt<Ur, Br>) -> Self::Output {
        self / rhs
    }
}

// -----------------------------------------
// Min
use crate::Min;

delegate_unsigned_binary_impls! {
    Min, min,
}

// -----------------------------------------
// Max
use crate::Max;

delegate_unsigned_binary_impls! {
    Max, max,
}

// -----------------------------------------
// SquareRoot

impl<N> SquareRoot for N
where
    N: PrivateSquareRoot,
{
    type Output = <Self as PrivateSquareRoot>::Output;
}

// sqrt(0) = 0.
impl PrivateSquareRoot for UTerm {
    type Output = UTerm;
}

// sqrt(1) = 1.
impl PrivateSquareRoot for UInt<UTerm, B1> {
    type Output = UInt<UTerm, B1>;
}

// General case of sqrt(Self) where Self >= 2. If a and b are
// bit-valued and Self = 4*u + 2*a + b, then the integer-valued
// (fractional part truncated) square root of Self is either 2*sqrt(u)
// or 2*sqrt(u)+1. Guess and check by comparing (2*sqrt(u)+1)^2
// against Self. Since the `typenum` result of that comparison is a
// bit, directly add that bit to 2*sqrt(u).
//
// Use `Sum<Double<Sqrt<U>>, GrEq<...>>` instead of `UInt<Sqrt<U>,
// GrEq<...>>` because `Sqrt<U>` can turn out to be `UTerm` and
// `GrEq<...>` can turn out to be `B0`, which would not be a valid
// UInt as leading zeros are disallowed.
impl<U, Ba, Bb> PrivateSquareRoot for UInt<UInt<U, Ba>, Bb>
where
    U: Unsigned,
    Ba: Bit,
    Bb: Bit,
    U: SquareRoot,
    Sqrt<U>: Shl<B1>,
    Double<Sqrt<U>>: Add<B1>,
    Add1<Double<Sqrt<U>>>: Mul,
    Self: IsGreaterOrEqual<Square<Add1<Double<Sqrt<U>>>>>,
    Double<Sqrt<U>>: Add<GrEq<Self, Square<Add1<Double<Sqrt<U>>>>>>,
{
    type Output = Sum<Double<Sqrt<U>>, GrEq<Self, Square<Add1<Double<Sqrt<U>>>>>>;
}

#[test]
fn sqrt_test() {
    use crate::consts::*;

    assert_eq!(0, <Sqrt<U0>>::to_u32());

    assert_eq!(1, <Sqrt<U1>>::to_u32());
    assert_eq!(1, <Sqrt<U2>>::to_u32());
    assert_eq!(1, <Sqrt<U3>>::to_u32());

    assert_eq!(2, <Sqrt<U4>>::to_u32());
    assert_eq!(2, <Sqrt<U5>>::to_u32());
    assert_eq!(2, <Sqrt<U6>>::to_u32());
    assert_eq!(2, <Sqrt<U7>>::to_u32());
    assert_eq!(2, <Sqrt<U8>>::to_u32());

    assert_eq!(3, <Sqrt<U9>>::to_u32());
    assert_eq!(3, <Sqrt<U10>>::to_u32());
    assert_eq!(3, <Sqrt<U11>>::to_u32());
    assert_eq!(3, <Sqrt<U12>>::to_u32());
    assert_eq!(3, <Sqrt<U13>>::to_u32());
    assert_eq!(3, <Sqrt<U14>>::to_u32());
    assert_eq!(3, <Sqrt<U15>>::to_u32());

    assert_eq!(4, <Sqrt<U16>>::to_u32());
    assert_eq!(4, <Sqrt<U17>>::to_u32());
    assert_eq!(4, <Sqrt<U18>>::to_u32());
    assert_eq!(4, <Sqrt<U19>>::to_u32());
    assert_eq!(4, <Sqrt<U20>>::to_u32());
    assert_eq!(4, <Sqrt<U21>>::to_u32());
    assert_eq!(4, <Sqrt<U22>>::to_u32());
    assert_eq!(4, <Sqrt<U23>>::to_u32());
    assert_eq!(4, <Sqrt<U24>>::to_u32());

    assert_eq!(5, <Sqrt<U25>>::to_u32());
    assert_eq!(5, <Sqrt<U26>>::to_u32());
    // ...
}

// -----------------------------------------
// Logarithm2

impl<N> Logarithm2 for N
where
    N: PrivateLogarithm2,
{
    type Output = <Self as PrivateLogarithm2>::Output;
}

// log2(1) = 0.
impl PrivateLogarithm2 for UInt<UTerm, B1> {
    type Output = U0;
}

// General case of log2(Self) where Self >= 2.
impl<U, B> PrivateLogarithm2 for UInt<U, B>
where
    U: Unsigned + Logarithm2,
    B: Bit,
    Log2<U>: Add<B1>,
{
    type Output = Add1<Log2<U>>;
}

// -----------------------------------------
// ToInt

impl<U: Unsigned> ToInt<i8> for U {
    #[inline]
    fn to_int() -> i8 {
        Self::I8
    }
    const INT: i8 = Self::I8;
}

impl<U: Unsigned> ToInt<i16> for U {
    #[inline]
    fn to_int() -> i16 {
        Self::I16
    }
    const INT: i16 = Self::I16;
}

impl<U: Unsigned> ToInt<i32> for U {
    #[inline]
    fn to_int() -> i32 {
        Self::I32
    }
    const INT: i32 = Self::I32;
}

impl<U: Unsigned> ToInt<i64> for U {
    #[inline]
    fn to_int() -> i64 {
        Self::I64
    }
    const INT: i64 = Self::I64;
}

impl<U: Unsigned> ToInt<isize> for U {
    #[inline]
    fn to_int() -> isize {
        Self::ISIZE
    }
    const INT: isize = Self::ISIZE;
}

#[cfg(feature = "i128")]
impl<U: Unsigned> ToInt<i128> for U {
    #[inline]
    fn to_int() -> i128 {
        Self::I128
    }
    const INT: i128 = Self::I128;
}

impl<U: Unsigned> ToInt<u8> for U {
    #[inline]
    fn to_int() -> u8 {
        Self::U8
    }
    const INT: u8 = Self::U8;
}

impl<U: Unsigned> ToInt<u16> for U {
    #[inline]
    fn to_int() -> u16 {
        Self::U16
    }
    const INT: u16 = Self::U16;
}

impl<U: Unsigned> ToInt<u32> for U {
    #[inline]
    fn to_int() -> u32 {
        Self::U32
    }
    const INT: u32 = Self::U32;
}

impl<U: Unsigned> ToInt<u64> for U {
    #[inline]
    fn to_int() -> u64 {
        Self::U64
    }
    const INT: u64 = Self::U64;
}

impl<U: Unsigned> ToInt<usize> for U {
    #[inline]
    fn to_int() -> usize {
        Self::USIZE
    }
    const INT: usize = Self::USIZE;
}

#[cfg(feature = "i128")]
impl<U: Unsigned> ToInt<u128> for U {
    #[inline]
    fn to_int() -> u128 {
        Self::U128
    }
    const INT: u128 = Self::U128;
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use crate::{Log2, ToInt, Unsigned};

    #[test]
    fn log2_test() {
        assert_eq!(0, <Log2<U1>>::to_u32());

        assert_eq!(1, <Log2<U2>>::to_u32());
        assert_eq!(1, <Log2<U3>>::to_u32());

        assert_eq!(2, <Log2<U4>>::to_u32());
        assert_eq!(2, <Log2<U5>>::to_u32());
        assert_eq!(2, <Log2<U6>>::to_u32());
        assert_eq!(2, <Log2<U7>>::to_u32());

        assert_eq!(3, <Log2<U8>>::to_u32());
        assert_eq!(3, <Log2<U9>>::to_u32());
        assert_eq!(3, <Log2<U10>>::to_u32());
        assert_eq!(3, <Log2<U11>>::to_u32());
        assert_eq!(3, <Log2<U12>>::to_u32());
        assert_eq!(3, <Log2<U13>>::to_u32());
        assert_eq!(3, <Log2<U14>>::to_u32());
        assert_eq!(3, <Log2<U15>>::to_u32());

        assert_eq!(4, <Log2<U16>>::to_u32());
        assert_eq!(4, <Log2<U17>>::to_u32());
        assert_eq!(4, <Log2<U18>>::to_u32());
        assert_eq!(4, <Log2<U19>>::to_u32());
        assert_eq!(4, <Log2<U20>>::to_u32());
        assert_eq!(4, <Log2<U21>>::to_u32());
        assert_eq!(4, <Log2<U22>>::to_u32());
        assert_eq!(4, <Log2<U23>>::to_u32());
        assert_eq!(4, <Log2<U24>>::to_u32());
        assert_eq!(4, <Log2<U25>>::to_u32());
        assert_eq!(4, <Log2<U26>>::to_u32());
        assert_eq!(4, <Log2<U27>>::to_u32());
        assert_eq!(4, <Log2<U28>>::to_u32());
        assert_eq!(4, <Log2<U29>>::to_u32());
        assert_eq!(4, <Log2<U30>>::to_u32());
        assert_eq!(4, <Log2<U31>>::to_u32());

        assert_eq!(5, <Log2<U32>>::to_u32());
        assert_eq!(5, <Log2<U33>>::to_u32());

        // ...
    }

    #[test]
    fn uint_toint_test() {
        // i8
        assert_eq!(0_i8, U0::to_int());
        assert_eq!(1_i8, U1::to_int());
        assert_eq!(2_i8, U2::to_int());
        assert_eq!(3_i8, U3::to_int());
        assert_eq!(4_i8, U4::to_int());
        assert_eq!(0_i8, U0::INT);
        assert_eq!(1_i8, U1::INT);
        assert_eq!(2_i8, U2::INT);
        assert_eq!(3_i8, U3::INT);
        assert_eq!(4_i8, U4::INT);

        // i16
        assert_eq!(0_i16, U0::to_int());
        assert_eq!(1_i16, U1::to_int());
        assert_eq!(2_i16, U2::to_int());
        assert_eq!(3_i16, U3::to_int());
        assert_eq!(4_i16, U4::to_int());
        assert_eq!(0_i16, U0::INT);
        assert_eq!(1_i16, U1::INT);
        assert_eq!(2_i16, U2::INT);
        assert_eq!(3_i16, U3::INT);
        assert_eq!(4_i16, U4::INT);

        // i32
        assert_eq!(0_i32, U0::to_int());
        assert_eq!(1_i32, U1::to_int());
        assert_eq!(2_i32, U2::to_int());
        assert_eq!(3_i32, U3::to_int());
        assert_eq!(4_i32, U4::to_int());
        assert_eq!(0_i32, U0::INT);
        assert_eq!(1_i32, U1::INT);
        assert_eq!(2_i32, U2::INT);
        assert_eq!(3_i32, U3::INT);
        assert_eq!(4_i32, U4::INT);

        // i64
        assert_eq!(0_i64, U0::to_int());
        assert_eq!(1_i64, U1::to_int());
        assert_eq!(2_i64, U2::to_int());
        assert_eq!(3_i64, U3::to_int());
        assert_eq!(4_i64, U4::to_int());
        assert_eq!(0_i64, U0::INT);
        assert_eq!(1_i64, U1::INT);
        assert_eq!(2_i64, U2::INT);
        assert_eq!(3_i64, U3::INT);
        assert_eq!(4_i64, U4::INT);

        // isize
        assert_eq!(0_isize, U0::to_int());
        assert_eq!(1_isize, U1::to_int());
        assert_eq!(2_isize, U2::to_int());
        assert_eq!(3_isize, U3::to_int());
        assert_eq!(4_isize, U4::to_int());
        assert_eq!(0_isize, U0::INT);
        assert_eq!(1_isize, U1::INT);
        assert_eq!(2_isize, U2::INT);
        assert_eq!(3_isize, U3::INT);
        assert_eq!(4_isize, U4::INT);

        // i128
        #[cfg(feature = "i128")]
        {
            assert_eq!(0_i128, U0::to_int());
            assert_eq!(1_i128, U1::to_int());
            assert_eq!(2_i128, U2::to_int());
            assert_eq!(3_i128, U3::to_int());
            assert_eq!(4_i128, U4::to_int());
            assert_eq!(0_i128, U0::INT);
            assert_eq!(1_i128, U1::INT);
            assert_eq!(2_i128, U2::INT);
            assert_eq!(3_i128, U3::INT);
            assert_eq!(4_i128, U4::INT);
        }

        // u8
        assert_eq!(0_u8, U0::to_int());
        assert_eq!(1_u8, U1::to_int());
        assert_eq!(2_u8, U2::to_int());
        assert_eq!(3_u8, U3::to_int());
        assert_eq!(4_u8, U4::to_int());
        assert_eq!(0_u8, U0::INT);
        assert_eq!(1_u8, U1::INT);
        assert_eq!(2_u8, U2::INT);
        assert_eq!(3_u8, U3::INT);
        assert_eq!(4_u8, U4::INT);

        // u16
        assert_eq!(0_u16, U0::to_int());
        assert_eq!(1_u16, U1::to_int());
        assert_eq!(2_u16, U2::to_int());
        assert_eq!(3_u16, U3::to_int());
        assert_eq!(4_u16, U4::to_int());
        assert_eq!(0_u16, U0::INT);
        assert_eq!(1_u16, U1::INT);
        assert_eq!(2_u16, U2::INT);
        assert_eq!(3_u16, U3::INT);
        assert_eq!(4_u16, U4::INT);

        // u32
        assert_eq!(0_u32, U0::to_int());
        assert_eq!(1_u32, U1::to_int());
        assert_eq!(2_u32, U2::to_int());
        assert_eq!(3_u32, U3::to_int());
        assert_eq!(4_u32, U4::to_int());
        assert_eq!(0_u32, U0::INT);
        assert_eq!(1_u32, U1::INT);
        assert_eq!(2_u32, U2::INT);
        assert_eq!(3_u32, U3::INT);
        assert_eq!(4_u32, U4::INT);

        // u64
        assert_eq!(0_u64, U0::to_int());
        assert_eq!(1_u64, U1::to_int());
        assert_eq!(2_u64, U2::to_int());
        assert_eq!(3_u64, U3::to_int());
        assert_eq!(4_u64, U4::to_int());
        assert_eq!(0_u64, U0::INT);
        assert_eq!(1_u64, U1::INT);
        assert_eq!(2_u64, U2::INT);
        assert_eq!(3_u64, U3::INT);
        assert_eq!(4_u64, U4::INT);

        // usize
        assert_eq!(0_usize, U0::to_int());
        assert_eq!(1_usize, U1::to_int());
        assert_eq!(2_usize, U2::to_int());
        assert_eq!(3_usize, U3::to_int());
        assert_eq!(4_usize, U4::to_int());
        assert_eq!(0_usize, U0::INT);
        assert_eq!(1_usize, U1::INT);
        assert_eq!(2_usize, U2::INT);
        assert_eq!(3_usize, U3::INT);
        assert_eq!(4_usize, U4::INT);

        // u128
        #[cfg(feature = "i128")]
        {
            assert_eq!(0_u128, U0::to_int());
            assert_eq!(1_u128, U1::to_int());
            assert_eq!(2_u128, U2::to_int());
            assert_eq!(3_u128, U3::to_int());
            assert_eq!(4_u128, U4::to_int());
            assert_eq!(0_u128, U0::INT);
            assert_eq!(1_u128, U1::INT);
            assert_eq!(2_u128, U2::INT);
            assert_eq!(3_u128, U3::INT);
            assert_eq!(4_u128, U4::INT);
        }
    }
}
