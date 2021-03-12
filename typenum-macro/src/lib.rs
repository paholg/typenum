#![no_std]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    parse_macro_input, LitInt, Token,
};

struct SignedInteger {
    has_negative_op: bool,
    value: u128,
}

impl Parse for SignedInteger {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let has_negative_op = if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            true
        } else {
            false
        };

        let literal = input.parse::<LitInt>()?;
        let value = literal.base10_parse::<u128>()?;

        let output = SignedInteger {
            has_negative_op,
            value,
        };

        Ok(output)
    }
}

struct UnsignedInteger {
    value: u128,
}

impl Parse for UnsignedInteger {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let literal = input.parse::<LitInt>()?;
        let value = literal.base10_parse::<u128>()?;

        let output = UnsignedInteger { value };

        Ok(output)
    }
}

#[proc_macro]
pub fn tyint(input: TokenStream) -> TokenStream {
    let SignedInteger {
        has_negative_op,
        value,
    } = parse_macro_input!(input as SignedInteger);

    let tokens = if value == 0 {
        quote! {
            typenum::consts::Z0
        }
    } else if has_negative_op {
        let uint_tokens = recursive_value_to_typeuint(value);
        quote! {
            typenum::int::NInt<#uint_tokens>
        }
    } else {
        let uint_tokens = recursive_value_to_typeuint(value);
        quote! {
            typenum::int::PInt<#uint_tokens>
        }
    };

    TokenStream::from(tokens)
}

#[proc_macro]
pub fn tyuint(input: TokenStream) -> TokenStream {
    let UnsignedInteger { value } = parse_macro_input!(input as UnsignedInteger);

    let tokens = recursive_value_to_typeuint(value);
    TokenStream::from(tokens)
}

fn recursive_value_to_typeuint(value: u128) -> TokenStream2 {
    if value == 0 {
        quote! {
            typenum::uint::UTerm
        }
    } else if value & 1 == 1 {
        let sub_tokens = recursive_value_to_typeuint(value >> 1);
        quote! {
            typenum::uint::UInt<#sub_tokens, typenum::bit::B1>
        }
    } else {
        let sub_tokens = recursive_value_to_typeuint(value >> 1);
        quote! {
            typenum::uint::UInt<#sub_tokens, typenum::bit::B0>
        }
    }
}
