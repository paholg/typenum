//! All of the **marker traits** used in typenum.
//!
//! Note that the definition here for marker traits is slightly different than the conventional one
//! -- we include traits with functions that convert a type to the corresponding value.
//!
//! For example, the `Integer` trait includes the function (among others) `fn to_i32() ->
//! i32` so that one can do this:
//!
//! ```
//! use typenum::{N42, Integer};
//!
//! assert_eq!(-42, N42::to_i32());
//! ```
//!
//!

/// A **marker trait** to designate that a type is not zero. All number types in this
/// crate implement `NonZero` except `B0`, `U0`, and `Z0`.
pub trait NonZero {}

/// A **Marker trait** for the types `Greater`, `Equal`, and `Less`.
///
/// This trait should not be implemented for anything outside this crate.
pub trait Ord {
    #[allow(missing_docs)]
    fn to_ordering() -> ::core::cmp::Ordering;
}

/// The **marker trait** for compile time bits.
///
/// This trait should not be implemented for anything outside this crate.
pub trait Bit {
    #[allow(missing_docs)]
    fn to_u8() -> u8;
    #[allow(missing_docs)]
    fn to_bool() -> bool;
}

/// The **marker trait** for compile time unsigned integers.
///
/// This trait should not be implemented for anything outside this crate.
///
/// # Example
/// ```rust
/// use typenum::{U3, Unsigned};
///
/// assert_eq!(U3::to_u32(), 3);
/// ```
pub trait Unsigned {
    #[allow(missing_docs)]
    fn to_u8() -> u8;
    #[allow(missing_docs)]
    fn to_u16() -> u16;
    #[allow(missing_docs)]
    fn to_u32() -> u32;
    #[allow(missing_docs)]
    fn to_u64() -> u64;
    #[cfg(feature="i128")]
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
    #[cfg(feature="i128")]
    #[allow(missing_docs)]
    fn to_i128() -> i128;
    #[allow(missing_docs)]
    fn to_isize() -> isize;
}

/// The **marker trait** for compile time signed integers.
///
/// This trait should not be implemented for anything outside this crate.
///
/// # Example
/// ```rust
/// use typenum::{P3, Integer};
///
/// assert_eq!(P3::to_i32(), 3);
/// ```
pub trait Integer {
    #[allow(missing_docs)]
    fn to_i8() -> i8;
    #[allow(missing_docs)]
    fn to_i16() -> i16;
    #[allow(missing_docs)]
    fn to_i32() -> i32;
    #[allow(missing_docs)]
    fn to_i64() -> i64;
    #[cfg(feature="i128")]
    #[allow(missing_docs)]
    fn to_i128() -> i128;
    #[allow(missing_docs)]
    fn to_isize() -> isize;
}

/// The **marker trait** for type-level arrays of type-level numbers.
///
/// This trait should not be implemented for anything outside this crate.
///
/// Someday, it will contain a function or associated constant to produce a runtime array, like the
/// other marker traits here. However, that requires stabilization of associated consts or of
/// const functions.
pub trait TypeArray {}
