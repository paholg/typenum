
mod bit;
mod uint;

pub trait Same<Rhs = Self> {
    /// `Output` should always be `Self`
    type Output;
}

pub trait Not {
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
