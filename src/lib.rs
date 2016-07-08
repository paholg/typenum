//! This crate provides type-level numbers evaluated at compile time. It depends only on libcore.
//!
//! The traits defined or used in this crate are used in a typical manner. They can be divided into
//! two categories: **marker traits** and **type operators**.
//!
//! Many of the marker traits have functions defined, but they all do essentially the same thing:
//! convert a type into its runtime counterpart, and are really just there for debugging. For
//! example,
//!
//! ```rust
//! use typenum::{N4, Integer};
//!
//! assert_eq!(N4::to_i32(), -4);
//! ```
//!
//! **Type operators** are traits that behave as functions at the type level. These are the meat of
//! this library. Where possible, traits defined in libcore have been used, but their attached
//! functions have not been implemented.
//!
//! For example, the `Add` trait is implemented for both unsigned and signed integers, but the
//! `add` function is not. As there are never any objects of the types defined here, it wouldn't
//! make sense to implement it. What is important is its associated type `Output`, which is where
//! the addition happens.
//!
//! ```rust
//! use std::ops::Add;
//! use typenum::{Integer, P3, P4};
//!
//! type X = <P3 as Add<P4>>::Output;
//! assert_eq!(<X as Integer>::to_i32(), 7);
//! ```
//!
//! In addition, helper aliases are defined for type operators. For example, the above snippet
//! could be replaced with
//!
//! ```rust
//! use typenum::{Sum, Integer, P3, P4};
//!
//! type X = Sum<P3, P4>;
//! assert_eq!(<X as Integer>::to_i32(), 7);
//! ```
//!
//! Documented in each module is the full list of type operators implemented.
//!

#![no_std]
#![warn(missing_docs)]

// For clippy:
#![allow(unknown_lints)]
#![allow(type_complexity, expl_impl_clone_on_copy)]

use core::cmp::Ordering;

macro_rules! impl_derivable {
    ($Type: ty) => (
        impl ::core::cmp::PartialEq for $Type {
            fn eq(&self, _: &Self) -> bool { match *self {} }
        }
        impl ::core::cmp::Eq for $Type { }
        impl ::core::cmp::PartialOrd for $Type {
            fn partial_cmp(&self, _: &Self) -> Option<::core::cmp::Ordering> { match *self {} }
        }
        impl ::core::cmp::Ord for $Type {
            fn cmp(&self, _: &Self) -> ::core::cmp::Ordering { match *self {} }
        }
        impl ::core::clone::Clone for $Type {
            fn clone(&self) -> Self { match *self {} }
        }
        impl ::core::marker::Copy for $Type {}
        impl ::core::hash::Hash for $Type {
            fn hash<H>(&self, _: &mut H) where H: ::core::hash::Hasher { match *self {} }
        }
        impl ::core::default::Default for $Type {
            fn default() -> Self { unreachable!() }
        }
        impl ::core::fmt::Debug for $Type {
            fn fmt(&self, _: &mut ::core::fmt::Formatter) ->
                ::core::result::Result<(), ::core::fmt::Error> {
                match *self {}
            }
        }
        );
}

pub mod consts;
pub mod bit;
pub mod uint;
pub mod int;
pub mod private;
pub mod marker_traits;
pub mod type_operators;
pub mod operator_aliases;

pub use consts::*;
pub use marker_traits::{NonZero, Ord, Bit, Unsigned, Integer};
pub use type_operators::{Cmp, Pow, Same};
pub use operator_aliases::{And, Or, Xor, Shleft, Shright, Sum, Diff, Prod, Quot, Mod, Negate, Exp,
                           Add1, Sub1, Square, Cube, Compare};
pub use uint::{UInt, UTerm};
pub use int::{NInt, PInt};

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Greater`.
pub enum Greater {}
impl_derivable!{Greater}

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Less`.
pub enum Less {}
impl_derivable!{Less}

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Equal`.
pub enum Equal {}
impl_derivable!{Equal}

/// Returns `core::cmp::Ordering::Greater`
impl Ord for Greater {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Greater
    }
}
/// Returns `core::cmp::Ordering::Less`
impl Ord for Less {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Less
    }
}
/// Returns `core::cmp::Ordering::Equal`
impl Ord for Equal {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Equal
    }
}
