use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Generates a string in the format of our `UInt`s from the given number.
/// Useful for generating type aliases.
pub fn gen_uint(u: u64) -> String {
    let bitstring = format!("{:b}", u);
    let bits = bitstring.chars().skip_while(|&c| c == '0');
    let mut uint = "UTerm".into();
    for bit in bits {
        uint = format!("UInt<{}, B{}>", uint, bit);
    }
    uint
}

/// Generates a string in the format of our `Int`s from the given number.
/// Useful for generating type aliases.
pub fn gen_int(i: i64) -> String {
    if i == 0 {
        "Z0".into()
    }
    else if i > 0 {
        format!("PInt<{}>", gen_uint(i as u64))
    }
    else {
        format!("NInt<{}>", gen_uint(i.abs() as u64))
    }
}

/// Runs the test strings. Expects output from the test functions in this module.
pub fn run_tests(tests: &[&str]) {
    use std::process::Command;
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_dir = Path::new(&out_dir).join("test/");
    let cargo = Path::new(&out_dir).join("test/Cargo.toml");
    let main = Path::new(&out_dir).join("test/src/main.rs");

    Command::new("cargo").arg("new").arg("--bin").arg(&test_dir).output().unwrap();

    // Write cargo file
    {
        let mut f = File::create(&cargo).unwrap();
        f.write(b"
[package]
name = \"test\"
version = \"0.0.1\"

[dependencies.typenum]
git = \"https://github.com/paholg/typenum\"
").unwrap();
    }

    // Write main.rs
    {
        let mut f = File::create(&main).unwrap();
        f.write(format!("
#![allow(unused_imports)]
extern crate typenum;

use std::ops::{{BitAnd, BitOr, BitXor, Shl, Shr, Neg, Add, Sub, Mul, Div, Rem}};
use typenum::{{NonZero, Same, Pow, Ord, Cmp, SizeOf}};
use typenum::bit::{{Bit, B0, B1}};
use typenum::uint::{{Unsigned, UInt, UTerm}};
use typenum::int::{{Integer, NInt, PInt, Z0}};

fn main() {{
    {}
}}
", tests.join("\n")).as_bytes()).unwrap();
    }

    let test_out = Command::new("cargo").arg("test").current_dir(&test_dir).output().unwrap();
    if !test_out.status.success() {
        let stdout = ::std::str::from_utf8(&test_out.stdout).unwrap();
        let stderr = ::std::str::from_utf8(&test_out.stderr).unwrap();
        panic!("Exit status: {}.\nStdout: {}\nStderr: {}\n", test_out.status, stdout, stderr);
    }
}

/// Ensures that `<A as Op<B>>::Output` is the same type as `Result` where
/// `A` is the type-level equivalent to 'a',
/// `B` is the type-level equivalent to 'b',
/// `Op` is trait named in 'op', which has associated type `Output`, and
/// `Result` is the type-level equivalent to 'result'
pub fn uint_binary_test(a: u64, op: &str, b: u64, result: u64) -> String {
    format!("
{{
    type A = {};
    type B = {};
    type Result = {};

    type Computed = <<A as {}<B>>::Output as Same<Result>>::Output;
    assert_eq!(<Computed as Unsigned>::to_u64(), <Result as Unsigned>::to_u64());
}}
", gen_uint(a), gen_uint(b), gen_uint(result), op)
}

/// Ensures that `<A as Op>::Output` is the same type as `Result` where
/// `A` is the type-level equivalent to 'a',
/// `Op` is trait named in 'op', which has associated type `Output`, and
/// `Result` is the type-level equivalent to 'result'
pub fn uint_unary_test(op: &str, a: u64, result: u64) -> String {
    format!("
{{
    type A = {};
    type Result = {};

    type Computed = <<A as {}>::Output as Same<Result>>::Output;
    assert_eq!(<Computed as Unsigned>::to_u64(), <Result as Unsigned>::to_u64());
}}
", gen_uint(a), gen_uint(result), op)
}

/// Ensures that `<A as Op<B>>::Output` is the same type as `Result` where
/// `A` is the type-level equivalent to 'a',
/// `B` is the type-level equivalent to 'b',
/// `Op` is trait named in 'op', which has associated type `Output`, and
/// `Result` is the type-level equivalent to 'result'
pub fn int_binary_test(a: i64, op: &str, b: i64, result: i64) -> String {
    format!("
{{
    type A = {};
    type B = {};
    type Result = {};

    type Computed = <<A as {}<B>>::Output as Same<Result>>::Output;
    assert_eq!(<Computed as Integer>::to_i64(), <Result as Integer>::to_i64());
}}
", gen_int(a), gen_int(b), gen_int(result), op)
}

/// Ensures that `<A as Op>::Output` is the same type as `Result` where
/// `A` is the type-level equivalent to 'a',
/// `Op` is trait named in 'op', which has associated type `Output`, and
/// `Result` is the type-level equivalent to 'result'
pub fn int_unary_test(op: &str, a: i64, result: i64) -> String {
    format!("
{{
    type A = {};
    type Result = {};

    type Computed = <<A as {}>::Output as Same<Result>>::Output;
    assert_eq!(<Computed as Integer>::to_i64(), <Result as Integer>::to_i64());
}}
", gen_int(a), gen_int(result), op)
}

// fixme: get a warning when testing without this
#[allow(dead_code)]
fn main() {
    // If you change this, change also the comments in src/consts.rs
    let highest: u64 = 1024;


    let first2: u32 = (highest as f64).log(2.0) as u32 + 1;
    let first10: u32 = (highest as f64).log(10.0) as u32 + 1;
    let uints = (0..(highest+1))
        .chain((first2..64).map(|i| 2u64.pow(i))) // powers of 2
        .chain((first10..20).map(|i| 10u64.pow(i))) // powers of 10
        ;


    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("consts.rs");

    let mut f = File::create(&dest).unwrap();

    // Header stuff here!
    let header = b"
use bit::{B0, B1};
use uint::{UInt, UTerm};
use int::{PInt, NInt};

pub use int::Z0; // re-export for convenience.
";
    f.write(header).unwrap();

    for u in uints {
        f.write(format!("pub type U{} = {};\n", u, gen_uint(u)).as_bytes()).unwrap();
        if u <= ::std::i64::MAX as u64 && u != 0 {
            let i = u as i64;
            f.write(format!("pub type P{} = {};\n", i, gen_int(i)).as_bytes()).unwrap();
            f.write(format!("pub type N{} = {};\n", i, gen_int(-i)).as_bytes()).unwrap();
        }
    }
}
