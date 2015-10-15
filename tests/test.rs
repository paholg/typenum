extern crate typenum;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fmt;
use std::process::Command;

use typenum::__private::build::{gen_int, gen_uint};

fn uint_binary_test(a: u64, op: &str, b: u64, result: u64) -> String {
    format!("
{{
    type A = {a};
    type B = {b};
    type {result_name} = {result};

    type {computed_name} = <<A as {op}<B>>::Output as Same<{result_name}>>::Output;
    assert_eq!(<{computed_name} as Unsigned>::to_u64(), <{result_name} as Unsigned>::to_u64());
}}
",
            a = gen_uint(a),
            b=gen_uint(b),
            result_name=format!("U{}", result),
            result=gen_uint(result),
            computed_name=format!("U{}_{}_U{}", a, op, b),
            op=op)
}

fn uint_unary_test(op: &str, a: u64, result: u64) -> String {
    format!("
{{
    type A = {a};
    type {result_name} = {result};

    type {computed_name} = <<A as {op}>::Output as Same<{result_name}>>::Output;
    assert_eq!(<{computed_name} as Unsigned>::to_u64(), <{result_name} as Unsigned>::to_u64());
}}
",
            a = gen_uint(a),
            result_name=format!("U{}", result),
            result=gen_uint(result),
            computed_name=format!("U{}_{}", a, op),
            op=op)
}

fn int_binary_test(a: i64, op: &str, b: i64, result: i64) -> String {
    let signa = if a > 0 { 'P' } else if a < 0 { 'N' } else { '_' };
    let signb = if b > 0 { 'P' } else if b < 0 { 'N' } else { '_' };
    let signr = if result > 0 { 'P' } else if result < 0 { 'N' } else { '_' };
    format!("
{{
    type A = {a};
    type B = {b};
    type {result_name} = {result};

    type {computed_name} = <<A as {op}<B>>::Output as Same<{result_name}>>::Output;
    assert_eq!(<{computed_name} as Integer>::to_i64(), <{result_name} as Integer>::to_i64());
}}
",
            a = gen_int(a),
            b=gen_int(b),
            result_name=format!("{}{}", signr, result.abs()),
            result=gen_int(result),
            computed_name=format!("{}{}_{}_{}{}", signa, a.abs(), op, signb, b.abs()),
            op=op)
}

fn int_unary_test(op: &str, a: i64, result: i64) -> String {
    let signa = if a > 0 { 'P' } else if a < 0 { 'N' } else { '_' };
    let signr = if result > 0 { 'P' } else if result < 0 { 'N' } else { '_' };
    format!("
{{
    type A = {a};
    type {result_name} = {result};

    type {computed_name} = <<A as {op}>::Output as Same<{result_name}>>::Output;
    assert_eq!(<{computed_name} as Integer>::to_i64(), <{result_name} as Integer>::to_i64());
}}
",
            a = gen_int(a),
            result_name=format!("{}{}", signr, result.abs()),
            result=gen_int(result),
            computed_name=format!("{}{}_{}", signa, a.abs(), op),
            op=op)
}

fn uint_cmp_test(a: u64, b: u64) -> String {
    format!("
{{
    type A = {a};
    type B = {b};
    type Result = {result:?};

    type {computed_name} = <A as Cmp<B>>::Output;
    assert_eq!(<{computed_name} as Ord>::to_ordering(), Ordering::{result:?});
}}
",
            a = gen_uint(a),
            b = gen_uint(b),
            result = a.cmp(&b),
            computed_name = format!("U{}_Cmp_U{}", a, b))
}

#[test]
fn test_all() {
    //let uints = (0..9).map(|a| );
    let uints = vec![(0, 0), (0, 1), (1, 0), (1, 1), (1, 2), (3, 4)];
    let ints = vec![(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1), (1, 2), (3, 4)];

    // uint operators: BitAnd, BitOr, BitXor, Shl, Shr, Add, Sub, Mul, Div, Pow, Cmp, SizeOf
    // fixme: add cmp and sizeof
    // let uint_tests = uints.iter().map(|&(a, b)| uint_binary_test(a, "BitAnd", b, a & b))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "BitOr", b, a | b)))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "BitXor", b, a ^ b)))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "Shl", b, a << b)))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "Shr", b, a >> b)))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "Add", b, a + b)))
    //     .chain(uints.iter().filter(|&&(a, b)| a >= b).map(|&(a, b)| uint_binary_test(a, "Sub", b, a - b)))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "Mul", b, a * b)))
    //     .chain(uints.iter().filter(|&&(_, b)| b != 0).map(|&(a, b)| uint_binary_test(a, "Div", b, a / b)))
    //     .chain(uints.iter().map(|&(a, b)| uint_binary_test(a, "Pow", b, a.pow(b as u32))))
    //     .chain(uints.iter().map(|&(a, b)| uint_cmp_test(a, b)))
    //     ;
    // run_tests(uint_tests.collect());

    // int operators: Neg, Add, Sub, Mul, Div, Pow, Cmp
    // let int_tests = ints.iter().map(|&(a, _)| int_unary_test("Neg", a, -a))
    //     .chain(ints.iter().map(|&(a, b)| int_binary_test(a, "Add", b, a + b)))
    //     .chain(ints.iter().map(|&(a, b)| int_binary_test(a, "Sub", b, a - b)))
    //     .chain(ints.iter().map(|&(a, b)| int_binary_test(a, "Mul", b, a * b)))
    //     .chain(ints.iter().filter(|&&(_, b)| b != 0).map(|&(a, b)| int_binary_test(a, "Div", b, a / b)))
    //     ;
    // run_tests(int_tests.collect());
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_dir = Path::new(&out_dir).join("test/");
    let cargo = Path::new(&out_dir).join("test/Cargo.toml");
    let main = Path::new(&out_dir).join("test/src/main.rs");

    Command::new("cargo").arg("new").arg("--bin").arg(&test_dir).output().unwrap();

    // Write cargo file
    let mut cargof = File::create(&cargo).unwrap();
    write!(cargof, "
[package]
name = \"test\"
version = \"0.0.1\"

[dependencies.typenum]
# typenum = \"0.1.0\"
git = \"file:{}\"
", env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();

    // Write main.rs
    let mut mainf = File::create(&main).unwrap();
    mainf.write(b"
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
extern crate typenum;

use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Neg, Add, Sub, Mul, Div, Rem};
use std::cmp::Ordering;
use typenum::{NonZero, Same, Pow, Ord, Cmp, SizeOf, Greater, Less, Equal};
use typenum::bit::{Bit, B0, B1};
use typenum::uint::{Unsigned, UInt, UTerm};
use typenum::int::{Integer, NInt, PInt, Z0};

fn main() {
    println!(\"testing! woohoo!\");
").unwrap();
    for (a, b) in uints {
        write!(mainf, "{}", uint_binary_test(a, "BitAnd", b, a & b)).unwrap();
        write!(mainf, "{}", uint_binary_test(a, "BitOr", b, a | b)).unwrap();
        write!(mainf, "{}", uint_binary_test(a, "BitXor", b, a ^ b)).unwrap();
        write!(mainf, "{}", uint_binary_test(a, "Shl", b, a << b)).unwrap();
        write!(mainf, "{}", uint_binary_test(a, "Shr", b, a >> b)).unwrap();
        write!(mainf, "{}", uint_binary_test(a, "Add", b, a + b)).unwrap();
        if a >= b {
            write!(mainf, "{}", uint_binary_test(a, "Sub", b, a - b)).unwrap();
        }
        write!(mainf, "{}", uint_binary_test(a, "Mul", b, a * b)).unwrap();
        if b != 0 {
            write!(mainf, "{}", uint_binary_test(a, "Div", b, a / b)).unwrap();
        }
        write!(mainf, "{}", uint_binary_test(a, "Pow", b, a.pow(b as u32))).unwrap();
    }
    mainf.write(b"}").unwrap();

    Command::new("cargo").arg("update").current_dir(&test_dir).output().unwrap();
    let test_out = Command::new("cargo").arg("run").current_dir(&test_dir).output().unwrap();
    if !test_out.status.success() {
        let stdout = ::std::str::from_utf8(&test_out.stdout).unwrap();
        let stderr = ::std::str::from_utf8(&test_out.stderr).unwrap();
        panic!("Exit status: {}.\nStdout: {}\nStderr: {}\n", test_out.status, stdout, stderr);
    }
}

