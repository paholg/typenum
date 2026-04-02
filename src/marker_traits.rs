//! All of the **marker traits** used in typenum.
//!
//! Note that the definition here for marker traits is slightly different than
//! the conventional one -- we include traits with functions that convert a type
//! to the corresponding value, as well as associated constants that do the
//! same.
//!
//! For example, the `Integer` trait includes the function (among others) `fn
//! to_i32() -> i32` and the associated constant `I32` so that one can do this:
//!
//! ```
//! use typenum::{Integer, N42};
//!
//! assert_eq!(-42, N42::to_i32());
//! assert_eq!(-42, N42::I32);
//! ```

use crate::sealed::Sealed;

/// A **marker trait** to designate that a type is not zero. All number types in this
/// crate implement `NonZero` except `B0`, `U0`, and `Z0`.
pub trait NonZero: Sealed {}

/// A **marker trait** to designate that a type is zero. Only `B0`, `U0`, and `Z0`
/// implement this trait.
pub trait Zero: Sealed {}

/// A **Marker trait** for the types `Greater`, `Equal`, and `Less`.
pub trait Ord: Sealed {
    /// Instantiates a singleton representing this ordering.
    fn new() -> Self;

    #[allow(missing_docs)]
    fn to_ordering() -> ::core::cmp::Ordering;

    /// Returns `B1` if `Self` is `Less`, `B0` otherwise.
    type IsLess: Bit;

    /// Returns `Rhs` if `Self` is `Equal`, `Self` otherwise.
    type Then<Rhs: Ord>: Ord;
    #[allow(missing_docs)]
    fn then<Rhs: Ord>() -> Self::Then<Rhs>;
}

/// The **marker trait** for compile time bits.
pub trait Bit: Sealed + Copy + Default + 'static {
    #[allow(missing_docs)]
    const U8: u8;
    #[allow(missing_docs)]
    const BOOL: bool;

    /// Negation of the current bit.
    type Not: Bit;

    /// Conjunction of the current bit with `Rhs`.
    type BitAnd<Rhs: Bit>: Bit;

    /// Disjunction of the current bit with `Rhs`.
    type BitOr<Rhs: Bit>: Bit;

    /// Exclusive or of the current bit with `Rhs`.
    type BitXor<Rhs: Bit>: Bit;

    /// Minimum between `Self` and `Rhs`.
    type Min<Rhs: Bit>: Bit;

    /// Maximum between `Self` and `Rhs`.
    type Max<Rhs: Bit>: Bit;

    /// Comparison between `Self` and `Rhs`.
    type Cmp<Rhs: Bit>: Ord;
    #[allow(missing_docs)]
    fn compare<Rhs: Bit>(rhs: Rhs) -> Self::Cmp<Rhs>;

    /// Returns `A` if `Self` is `B1`, `B` otherwise. `A`, `B` and the output must implement `Ord`.
    type IfOrd<A: Ord, B: Ord>: Ord;
    #[allow(missing_docs)]
    fn if_ord<A: Ord, B: Ord>(a: A, b: B) -> Self::IfOrd<A, B>;

    /// Returns `A` if `Self` is `B1`, `B` otherwise. `A`, `B` and the output must implement `Unsigned`.
    type IfUnsigned<A: Unsigned, B: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn if_unsigned<A: Unsigned, B: Unsigned>(a: A, b: B) -> Self::IfUnsigned<A, B>;

    /// Returns `S << N` if `Self` is B0, `S` otherwise.
    type SelectShlUnsigned<S: Unsigned, N: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn select_shl_unsigned<S: Unsigned, N: Unsigned>(s: S, n: N) -> Self::SelectShlUnsigned<S, N>;

    /// Instantiates a singleton representing this bit.
    fn new() -> Self;

    #[allow(missing_docs)]
    fn to_u8() -> u8;
    #[allow(missing_docs)]
    fn to_bool() -> bool;
}

/// The **marker trait** for compile time unsigned integers.
///
/// # Example
/// ```rust
/// use typenum::{Unsigned, U3};
///
/// assert_eq!(U3::to_u32(), 3);
/// assert_eq!(U3::I32, 3);
/// ```
pub trait Unsigned: Sealed + Copy + Default + 'static {
    #[allow(missing_docs)]
    const U8: u8;
    #[allow(missing_docs)]
    const U16: u16;
    #[allow(missing_docs)]
    const U32: u32;
    #[allow(missing_docs)]
    const U64: u64;
    #[cfg(feature = "i128")]
    #[allow(missing_docs)]
    const U128: u128;
    #[allow(missing_docs)]
    const USIZE: usize;

    #[allow(missing_docs)]
    const I8: i8;
    #[allow(missing_docs)]
    const I16: i16;
    #[allow(missing_docs)]
    const I32: i32;
    #[allow(missing_docs)]
    const I64: i64;
    #[cfg(feature = "i128")]
    #[allow(missing_docs)]
    const I128: i128;
    #[allow(missing_docs)]
    const ISIZE: isize;

    #[allow(missing_docs)]
    fn to_u8() -> u8;
    #[allow(missing_docs)]
    fn to_u16() -> u16;
    #[allow(missing_docs)]
    fn to_u32() -> u32;
    #[allow(missing_docs)]
    fn to_u64() -> u64;
    #[cfg(feature = "i128")]
    #[allow(missing_docs)]
    fn to_u128() -> u128;
    #[allow(missing_docs)]
    fn to_usize() -> usize;

    #[allow(missing_docs)]
    fn to_i8() -> i8;
    #[allow(missing_docs)]
    fn to_i16() -> i16;
    #[allow(missing_docs)]
    fn to_i32() -> i32;
    #[allow(missing_docs)]
    fn to_i64() -> i64;
    #[cfg(feature = "i128")]
    #[allow(missing_docs)]
    fn to_i128() -> i128;
    #[allow(missing_docs)]
    fn to_isize() -> isize;

    /// Get the Most Significant Bits as a type.
    type GetMSB: Unsigned;
    #[allow(missing_docs)]
    fn get_msb(self) -> Self::GetMSB;

    /// Get the Least Significant Bit as a type.
    type GetLSB: Bit;
    #[allow(missing_docs)]
    fn get_lsb(self) -> Self::GetLSB;

    /// Successor of `Self`, equal to `Self + 1`
    type Successor: Unsigned;
    #[allow(missing_docs)]
    fn successor(self) -> Self::Successor;

    /// Predecessor of `Self`, equal to `Self - 1`
    type Predecessor: Unsigned;
    #[allow(missing_docs)]
    fn predecessor(self) -> Self::Predecessor;

    /// Trim `Self`, removing leading 0 bits.
    type Trimmed: Unsigned;
    #[allow(missing_docs)]
    fn trimmed(self) -> Self::Trimmed;

    /// Returns `B1` if `Self` is zero.
    type IsZero: Bit;
    #[allow(missing_docs)]
    fn is_zero(self) -> Self::IsZero;

    /// Returns `B1` if `Self` is even.
    type IsEven: Bit;
    #[allow(missing_docs)]
    fn is_even(self) -> Self::IsEven;

    /// Returns `B1` if `Self` is odd.
    type IsOdd: Bit;
    #[allow(missing_docs)]
    fn is_odd(self) -> Self::IsOdd;

    /// Minimum between `Self` and `Rhs`.
    type Min<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn min<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Min<Rhs>;

    /// Maximum between `Self` and `Rhs`.
    type Max<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn max<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Max<Rhs>;

    /// Returns `Self & Rhs`.
    type BitAnd<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn bitand<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitAnd<Rhs>;

    /// Returns `Self | Rhs`.
    type BitOr<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn bitor<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitOr<Rhs>;

    /// Returns `Self ^ Rhs`.
    type BitXor<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn bitxor<Rhs: Unsigned>(self, rhs: Rhs) -> Self::BitXor<Rhs>;

    /// Add `Self` with `Rhs`.
    type Add<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn add<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Add<Rhs>;
    /// Add `Self` with `Rhs`, with an additional carry bit.
    type AddCarry<Rhs: Unsigned, Carry: Bit>: Unsigned;
    #[allow(missing_docs)]
    fn add_carry<Rhs: Unsigned, Carry: Bit>(self, rhs: Rhs) -> Self::AddCarry<Rhs, Carry>;

    /// Multiply `Self` with `Rhs`.
    type Mul<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn mul<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Mul<Rhs>;

    /// Computes `Self**Rhs`.
    type Pow<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn powi<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Pow<Rhs>;
    /// Computes `Lhs**Self`.
    type PowSelf<Lhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn powi_self<Lhs: Unsigned>(self, lhs: Lhs) -> Self::PowSelf<Lhs>;

    /// Shift `Self` right by `Rhs`.
    type Shr<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn shr<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Shr<Rhs>;

    /// Shift `Self` left by `Rhs`.
    type Shl<Rhs: Unsigned>: Unsigned;
    #[allow(missing_docs)]
    fn shl<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Shl<Rhs>;

    /// Returns `Self * 2 = Self << 1`.
    type Double: Unsigned;
    #[allow(missing_docs)]
    fn double(self) -> Self::Double;

    /// Compare `Self` with `Rhs`.
    type Cmp<Rhs: Unsigned>: Ord;
    #[allow(missing_docs)]
    fn compare<Rhs: Unsigned>(self, rhs: Rhs) -> Self::Cmp<Rhs>;
}

/// The **marker trait** for converting a type to an unsigned integer at compile time.
pub trait IntoUnsigned {
    /// The type converted to an unsigned integer.
    type IntoUnsigned: Unsigned;
    #[allow(missing_docs)]
    fn into_unsigned(self) -> Self::IntoUnsigned;
}
impl<U: Unsigned> IntoUnsigned for U {
    type IntoUnsigned = Self;

    #[inline]
    fn into_unsigned(self) -> Self::IntoUnsigned {
        self
    }
}

/// The **marker trait** for compile time signed integers.
///
/// # Example
/// ```rust
/// use typenum::{Integer, P3};
///
/// assert_eq!(P3::to_i32(), 3);
/// assert_eq!(P3::I32, 3);
/// ```
pub trait Integer: Sealed + Copy + Default + 'static {
    #[allow(missing_docs)]
    const I8: i8;
    #[allow(missing_docs)]
    const I16: i16;
    #[allow(missing_docs)]
    const I32: i32;
    #[allow(missing_docs)]
    const I64: i64;
    #[cfg(feature = "i128")]
    #[allow(missing_docs)]
    const I128: i128;
    #[allow(missing_docs)]
    const ISIZE: isize;

    #[allow(missing_docs)]
    fn to_i8() -> i8;
    #[allow(missing_docs)]
    fn to_i16() -> i16;
    #[allow(missing_docs)]
    fn to_i32() -> i32;
    #[allow(missing_docs)]
    fn to_i64() -> i64;
    #[cfg(feature = "i128")]
    #[allow(missing_docs)]
    fn to_i128() -> i128;
    #[allow(missing_docs)]
    fn to_isize() -> isize;
}

/// The **marker trait** for type-level arrays of type-level numbers.
///
/// Someday, it may contain an associated constant to produce a runtime array,
/// like the other marker traits here. However, that is blocked by [this
/// issue](https://github.com/rust-lang/rust/issues/44168).
pub trait TypeArray: Sealed {}

/// The **marker trait** for type-level numbers which are a power of two.
///
/// # Examples
///
/// Here's a working example:
///
/// ```rust
/// use typenum::{PowerOfTwo, P4, P8};
///
/// fn only_p2<P: PowerOfTwo>() {}
///
/// only_p2::<P4>();
/// only_p2::<P8>();
/// ```
///
/// Numbers which are not a power of two will fail to compile in this example:
///
/// ```rust,compile_fail
/// use typenum::{P9, P511, P1023, PowerOfTwo};
///
/// fn only_p2<P: PowerOfTwo>() { }
///
/// only_p2::<P9>();
/// only_p2::<P511>();
/// only_p2::<P1023>();
/// ```
pub trait PowerOfTwo: Sealed {}
