/// A **marker trait** to designate that a type is not zero. All number types in this
/// crate implement `NonZero` except `B0`, `U0`, and `Z0`.
pub trait NonZero {}

/**

A **Marker trait** for the types `Greater`, `Equal`, and `Less`.

This trait should not be implemented for anything outside this crate.

*/
pub trait Ord {
    #[allow(missing_docs)]
    fn to_ordering() -> ::core::cmp::Ordering;
}

/**
The **marker trait** for compile time bits.

This trait should not be implemented for anything outside this crate.
*/
pub trait Bit {
    #[allow(missing_docs)]
    fn to_u8() -> u8;
    #[allow(missing_docs)]
    fn to_bool() -> bool;
}

/**
The **marker trait** for compile time unsigned integers.

This trait should not be implemented for anything outside this crate.

# Example
```rust
use typenum::consts::U3;
use typenum::uint::Unsigned;

assert_eq!(U3::to_u32(), 3);
```
*/
pub trait Unsigned {
    #[allow(missing_docs)]
    fn to_u8() -> u8;
    #[allow(missing_docs)]
    fn to_u16() -> u16;
    #[allow(missing_docs)]
    fn to_u32() -> u32;
    #[allow(missing_docs)]
    fn to_u64() -> u64;
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
    #[allow(missing_docs)]
    fn to_isize() -> isize;
}

/**
The **marker trait** for compile time signed integers.

This trait should not be implemented for anything outside this crate.

# Example
```rust
use typenum::consts::P3;
use typenum::int::Integer;

assert_eq!(P3::to_i32(), 3);
```
*/
pub trait Integer {
    #[allow(missing_docs)]
    fn to_i8() -> i8;
    #[allow(missing_docs)]
    fn to_i16() -> i16;
    #[allow(missing_docs)]
    fn to_i32() -> i32;
    #[allow(missing_docs)]
    fn to_i64() -> i64;
    #[allow(missing_docs)]
    fn to_isize() -> isize;
}
