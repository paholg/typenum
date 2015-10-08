use std::cmp::{Ordering};

pub mod bit;

pub mod uint;
pub mod consts;

pub mod int;

pub mod __private;

pub trait Same<Rhs = Self> {
    /// `Output` should always be `Self`
    type Output;
}

pub trait Not {
    type Output;
}
pub trait Neg {
    type Output;
}
pub trait And<Rhs = Self> {
    type Output;
}
pub trait Or<Rhs = Self> {
    type Output;
}
pub trait Xor<Rhs = Self> {
    type Output;
}

pub trait Shl<Rhs = Self> {
    type Output;
}
pub trait Shr<Rhs = Self> {
    type Output;
}

pub trait Add<Rhs = Self> {
    type Output;
}
pub trait Sub<Rhs = Self> {
    type Output;
}
pub trait Mul<Rhs = Self> {
    type Output;
}
pub trait Div<Rhs = Self> {
    type Output;
}
pub trait Rem<Rhs = Self> {
    type Output;
}

pub trait Ord {
    fn to_ordering() -> Ordering;
}

pub struct Greater;
pub struct Less;
pub struct Equal;

impl Ord for Greater {
    fn to_ordering() -> Ordering { Ordering::Greater }
}
impl Ord for Less {
    fn to_ordering() -> Ordering { Ordering::Less }
}
impl Ord for Equal {
    fn to_ordering() -> Ordering { Ordering::Equal }
}

/// Compares `Self` and `Rhs`. Should only ever return one of `Greater`, `Less`, or `Equal`.
pub trait Cmp<Rhs = Self> {
    type Output;
}
