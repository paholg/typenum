use super::*;

pub fn emit_impls() -> String {
    let mut result = String::new();
    result.push_str("
//! Module with some `const`-generics-friendly definitions, to help bridge the gap
//! between those and `typenum` types.
//!
//!   - It requires the `const-generics` crate feature to be enabled.
//!
//! The main type to use here is [`U`], although [`Const`] and [`ToUInt`] may be needed
//! in a generic context.

use crate::*;

/// The main mapping from a generic `const: usize` to a [`UInt`]: [`U<N>`] is expected to work like [`UN`].
///
///   - It requires the `const-generics` crate feature to be enabled.
///
/// [`U<N>`]: `U`
/// [`UN`]: `U42`
///
/// # Example
///
/// ```rust
/// use typenum::*;
///
/// assert_type_eq!(U<42>, U42);
/// ```
///
/// This can even be used in a generic `const N: usize` context, provided the
/// genericity is guarded by a `where` clause:
///
/// ```rust
/// use typenum::*;
///
/// struct MyStruct<const N: usize>;
///
/// trait MyTrait { type AssocType; }
///
/// impl<const N: usize> MyTrait
///     for MyStruct<N>
/// where
///     Const<N> : ToUInt,
/// {
///     type AssocType = U<N>;
/// }
///
/// assert_type_eq!(<MyStruct<42> as MyTrait>::AssocType, U42);
/// ```
pub type U<const N: usize> = <Const<N> as ToUInt>::Output;

/// Used to allow the usage of [`U`] in a generic context.
pub struct Const<const N: usize>;

/// Used to allow the usage of [`U`] in a generic context.
pub trait ToUInt {
    /// The [`UN`][`crate::U42`] type corresponding to `Self = Const<N>`.
    type Output;
}
");

    for uint in uints() {
        result.push_str(&format!(
            "
    {cfg}
    impl ToUInt for Const<{uint}> {{
        type Output = U{uint};
    }}
\
            ",
            uint = uint,
            cfg = feature_gate_to_64_bit(uint),
        ));
    }

    result
}

const fn feature_gate_to_64_bit(uint: u64) -> &'static str {
    if uint > u32::MAX as u64 {
        r#"#[cfg(target_pointer_width = "64")]"#
    } else {
        ""
    }
}
