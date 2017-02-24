//! This module provides aliases for the type operators used in this crate. Their purpose is
//! to increase the ergonomics of performing operations on the types defined here.
//!
//! For example, type `X` and type `Y` are the same here:
//!
//! ```rust
//! use std::ops::Mul;
//! use typenum::{Prod, P5, P7};
//!
//! type X = <P7 as Mul<P5>>::Output;
//! type Y = Prod<P7, P5>;
//! ```
//!
//!
//! Aliases!!!
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul, Div, Rem, Neg};
use type_operators::{Pow, Cmp, Len, PartialDiv};

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

/// Alias for the associated type of `PartialDiv`: `PartialQuot<A, B> = <A as PartialDiv<B>>::Output`
pub type PartialQuot<A, B> = <A as PartialDiv<B>>::Output;

/// Alias for the associated type of `Neg`: `Negate<A> = <A as Neg>::Output`
pub type Negate<A> = <A as Neg>::Output;

/// Alias for the associated type of `Pow`: `Exp<A, B> = <A as Pow<B>>::Output`
pub type Exp<A, B> = <A as Pow<B>>::Output;


/// Alias to make it easy to add 1: `Add1<A> = <A as Add<B1>>::Output`
pub type Add1<A> = <A as Add<::bit::B1>>::Output;
/// Alias to make it easy to subtract 1: `Sub1<A> = <A as Sub<B1>>::Output`
pub type Sub1<A> = <A as Sub<::bit::B1>>::Output;

/// Alias to make it easy to square. `Square<A> = <A as Mul<A>>::Output`
pub type Square<A> = <A as Mul>::Output;
/// Alias to make it easy to square. `Cube<A> = <Square<A> as Mul<A>>::Output`
pub type Cube<A> = <Square<A> as Mul<A>>::Output;

/// Alias for the associated type of `Cmp`: `Compare<A, B> = <A as Cmp<B>>::Output`
pub type Compare<A, B> = <A as Cmp<B>>::Output;

/// Alias for the associated type of `Len`: `Length<A> = <A as Len>::Output`
pub type Length<T> = <T as Len>::Output;
