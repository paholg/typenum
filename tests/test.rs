extern crate typenum;
extern crate quickcheck;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use quickcheck::quickcheck;

use typenum::build::{gen_uint, gen_int};

/// Ensures that `<A as Op<B>>::Output` is the same type as `Result` where
/// `A` is the type-level equivalent to 'a',
/// `B` is the type-level equivalent to 'b',
/// `Op` is trait named in 'op', which has associated type `Output`, and
/// `Result` is the type-level equivalent to 'result'
fn uint_binary_test(a: u64, op: &str, b: u64, result: u64) -> String {
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
fn uint_unary_test(op: &str, a: u64, result: u64) -> String {
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
fn int_binary_test(a: i64, op: &str, b: i64, result: i64) -> String {
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
fn int_unary_test(op: &str, a: i64, result: i64) -> String {
    format!("
{{
    type A = {};
    type Result = {};

    type Computed = <<A as {}>::Output as Same<Result>>::Output;
    assert_eq!(<Computed as Integer>::to_i64(), <Result as Integer>::to_i64());
}}
", gen_int(a), gen_int(result), op)
}

/// Runs the test strings. Expects output from the test functions in this module.
fn run_tests(tests: Vec<String>) -> bool {
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
    println!(\"testing! woohoo!\");
    {}
}}
", tests.join("\n")).as_bytes()).unwrap();
    }

    let test_out = Command::new("cargo").arg("run").current_dir(&test_dir).output().unwrap();
    if !test_out.status.success() {
        let stdout = ::std::str::from_utf8(&test_out.stdout).unwrap();
        let stderr = ::std::str::from_utf8(&test_out.stderr).unwrap();
        println!("Exit status: {}.\nStdout: {}\nStderr: {}\n", test_out.status, stdout, stderr);
        return false;
    }
    true
}

#[test]
fn sample_runtime_tests() {
    let tests = vec![
        uint_binary_test(7, "Div", 2, 3),
        uint_unary_test("SizeOf", 17, 5),

        int_binary_test(7, "Add", 2, 9),
        int_unary_test("Neg", 3, -3)
            ];
    run_tests(tests);
}

#[test]
fn test_all() {
    fn test_ints(inputs: Vec<(i64, i64)>) -> bool {
        let tests = inputs.iter().map(|&(a, b)| int_binary_test(a, "Add", b, a + b))
        .chain(inputs.iter().map(|&(a, b)| int_binary_test(a, "Sub", b, a - b)))
        .chain(inputs.iter().map(|&(a, b)| int_binary_test(a, "Mul", b, a * b)));
        //.chain(inputs.iter().map(|&(a, b)| uint_binary_test(a, "Div", b, a / b)));
        run_tests(tests.collect())
    }
    quickcheck(test_ints as fn(Vec<(i64, i64)>) -> bool);
}

