/// Generates a signed typenum from a signed integer literal.
///
/// The output type can be either [PInt](typenum::PInt), [NInt](typenum::NInt) or [Z0](typenum::Z0),
/// (e.g., [P1](typenum::P1), [N2](typenum::N2)) depending on the value of input literal.
///
/// ```rust
/// use typenum::{tyint, N2, P1, Z0};
/// let _: tyint!(0) = Z0::new();
/// let _: tyint!(1) = P1::new();
/// let _: tyint!(-2) = N2::new();
/// ```
pub use typenum_macro::tyint;

/// Generates an unsigned typenum from an unsigned integer literal.
///
/// The output type can be either [UTerm](typenum::UTerm) or [UInt](typenum::UInt)
/// (e.g., [U0](typenum::U0), [U1](typenum::U1)) depending on the value of input literal.
///
/// ```rust
/// use typenum::{tyuint, U0, U1, U2};
/// let _: tyuint!(0) = U0::new();
/// let _: tyuint!(1) = U1::new();
/// let _: tyuint!(2) = U2::new();
/// ```
pub use typenum_macro::tyuint;
