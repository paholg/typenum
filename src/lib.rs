/*!
This crate provides type-level numbers evaluated at compile time.

The traits defined or used in this crate are used in a typical manner. They can
be divided into two categories: **marker traits** and **type operators**.

Many of the marker traits have functions defined, but they all do essentially the same
thing: convert a type into its runtime counterpart, and are really just there for
debugging. For example,

```rust
# use typenum::consts::N4;
# use typenum::int::Integer;
assert_eq!(N4::to_i32(), -4);
```

*Type operators** are traits that behave as functions at the type level. These are the
meat of this library. Where possible, traits defined in the stdlib have been used, but
their attached functions have not been implemented.

For example, the `Add` trait is implemented for both unsigned and signed integers, but
the `add` function is not. As there are never any objects of the types defined here, it
wouldn't make sense to implement it. What is important is its associated type `Output`,
which is where the addition happens.

```rust
use std::ops::Add;
use typenum::consts::{P3, P4};
use typenum::int::Integer;

type X = <P3 as Add<P4>>::Output;
assert_eq!(<X as Integer>::to_i32(), 7);
```

Documented in each module is the full list of type operators implemented.
*/
#![cfg_attr(feature="no_std", feature(core, no_std))]
#![cfg_attr(feature="no_std", no_std)]
#[cfg(feature="no_std")]
extern crate core as std;

use std::cmp::Ordering;

pub mod consts;
pub mod bit;
pub mod uint;
pub mod int;
pub mod __private;

/// A **marker trait** to designate that a type is not zero. All number types in this
/// crate implement `NonZero` except `B0`, `U0`, and `Z0`.
pub trait NonZero {}

/**
A **type operator** that ensures that `Rhs` is the same as `Self`, it is mainly useful
for writing macros that can take arbitrary binary or unary operators.

`Same` is implemented generically for all types; it should never need to be implemented
for anything else.

Note that Rust lazily evaluates types, so this will only fail for two different types if
the `Output` is used.

# Example
```rust
use typenum::Same;
use typenum::consts::{U4, U5};
use typenum::uint::Unsigned;

assert_eq!(<U5 as Same<U5>>::Output::to_u32(), 5);

// Only an error if we use it:
type Undefined = <U5 as Same<U4>>::Output;
// Compiler error:
// Undefined::to_u32();
```
*/
pub trait Same<Rhs = Self> {
    /// Should always be `Self`
    type Output;
}

impl<T> Same<T> for T {
    type Output = T;
}

/**
A **type operator** that provides exponentiation by repeated squaring.

# Example
```rust
use typenum::Pow;
use typenum::int::Integer;
use typenum::consts::{N3, P3};

assert_eq!(<N3 as Pow<P3>>::Output::to_i32(), -27);
```
*/
pub trait Pow<Rhs = Self> {
    type Output;
}

/**

A **Marker trait** for the types `Greater`, `Equal`, and `Less`.

This trait should not be implemented for anything outside this crate.

*/
pub trait Ord {
    fn to_ordering() -> Ordering;
}

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `std::cmp::Ordering::Greater`.
pub enum Greater {}
/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `std::cmp::Ordering::Less`.
pub enum Less {}
/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `std::cmp::Ordering::Equal`.
pub enum Equal {}

/// Returns `std::cmp::Ordering::Greater`
impl Ord for Greater {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Greater
    }
}
/// Returns `std::cmp::Ordering::Less`
impl Ord for Less {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Less
    }
}
/// Returns `std::cmp::Ordering::Equal`
impl Ord for Equal {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Equal
    }
}

/**
A **type operator** for comparing `Self` and `Rhs`. It provides a similar functionality to
the function [`std::cmp::Ord::cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp) but for types.

# Example
```rust
use typenum::{Cmp, Ord, Greater, Less, Equal};
use typenum::consts::{N3, P2, P5};

assert_eq!(<P2 as Cmp<N3>>::Output::to_ordering(), Greater::to_ordering());
assert_eq!(<P2 as Cmp<P2>>::Output::to_ordering(), Equal::to_ordering());
assert_eq!(<P2 as Cmp<P5>>::Output::to_ordering(), Less::to_ordering());
```
*/
pub trait Cmp<Rhs = Self> {
    /// The result of the comparison. It should only ever be one of `Greater`, `Less`, or `Equal`.
    type Output;
}



// Aliases!!!
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul, Div, Rem, Neg};

/// Alias for the associated type of `BitAnd`: `And<A, B> = <A as BitAnd<B>>::Output`
pub type And<A, B> = <A as BitAnd<B>>::Output;
/// Alias for the associated type of `BitOr`: `Or<A, B> = <A as BitOr<B>>::Output`
pub type Or<A, B> = <A as BitOr<B>>::Output;
/// Alias for the associated type of `BitXor`: `Xor<A, B> = <A as BitXor<B>>::Output`
pub type Xor<A, B> = <A as BitXor<B>>::Output;

/// Alias for the associated type of `Shl`: `Shleft<A, B> = <A as Shl<B>>::Output`
pub type Shleft<A, B> = <A as Shl<B>>::Output;
/// Alias for the associated type of `Shr`: `Shright<A, B> = <A as Shr<B>>::Output`
pub type Shright<A, B> = <A as Shr<B>>::Output;


/// Alias for the associated type of `Add`: `Sum<A, B> = <A as Add<B>>::Output`
pub type Sum<A, B> = <A as Add<B>>::Output;
/// Alias for the associated type of `Sub`: `Diff<A, B> = <A as Sub<B>>::Output`
pub type Diff<A, B> = <A as Sub<B>>::Output;
/// Alias for the associated type of `Mul`: `Prod<A, B> = <A as Mul<B>>::Output`
pub type Prod<A, B> = <A as Mul<B>>::Output;
/// Alias for the associated type of `Div`: `Quot<A, B> = <A as Div<B>>::Output`
pub type Quot<A, B> = <A as Div<B>>::Output;
/// Alias for the associated type of `Rem`: `Mod<A, B> = <A as Rem<B>>::Output`
pub type Mod<A, B> = <A as Rem<B>>::Output;

/// Alias for the associated type of `Neg`: `Negate<A> = <A as Neg>::Output`
pub type Negate<A> = <A as Neg>::Output;

/// Alias for the associated type of `Pow`: `Exp<A, B> = <A as Pow<B>>::Output`
pub type Exp<A, B> = <A as Pow<B>>::Output;


/// Alias to make it easy to add 1: `Add1<A> = <A as Add<B1>>::Output`
pub type Add1<A> = <A as Add<bit::B1>>::Output;
/// Alias to make it easy to subtract 1: `Sub1<A> = <A as Sub<B1>>::Output`
pub type Sub1<A> = <A as Sub<bit::B1>>::Output;

/// Alias to make it easy to multiply by `U2`: `MulU2<A> = <A as Mul<U2>>::Output`
pub type MulU2<A> = <A as Mul<consts::U2>>::Output;
/// Alias to make it easy to multiply by `U10`: `MulU10<A> = <A as Mul<U10>>::Output`
pub type MulU10<A> = <A as Mul<consts::U10>>::Output;

/// Alias to make it easy to multiply by `P2`: `MulP2<A> = <A as Mul<P2>>::Output`
pub type MulP2<A> = <A as Mul<consts::P2>>::Output;
/// Alias to make it easy to multiply by `P10`: `MulP10<A> = <A as Mul<P10>>::Output`
pub type MulP10<A> = <A as Mul<consts::P10>>::Output;

/// Alias to make it easy to divide by `U2`: `DivU2<A> = <A as Div<U2>>::Output`
pub type DivU2<A> = <A as Div<consts::U2>>::Output;
/// Alias to make it easy to divide by `U10`: `DivU10<A> = <A as Div<U10>>::Output`
pub type DivU10<A> = <A as Div<consts::U10>>::Output;

/// Alias to make it easy to divide by `P2`: `DivP2<A> = <A as Div<P2>>::Output`
pub type DivP2<A> = <A as Div<consts::P2>>::Output;
/// Alias to make it easy to divide by `P10`: `DivP10<A> = <A as Div<P10>>::Output`
pub type DivP10<A> = <A as Div<consts::P10>>::Output;

/// Alias to make it easy to square. `Square<A> = <A as Mul<A>>::Output`
pub type Square<A> = <A as Mul>::Output;
/// Alias to make it easy to square. `Cube<A> = <Square<A> as Mul<A>>::Output`
pub type Cube<A> = <Square<A> as Mul<A>>::Output;

/// Alias for the associated type of `Cmp`: `Compare<A, B> = <A as Cmp<B>>::Output`
pub type Compare<A, B> = <A as Cmp<B>>::Output;
