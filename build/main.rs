use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fmt;

mod tests;

pub enum UIntCode {
    Term,
    Zero(Box<UIntCode>),
    One(Box<UIntCode>),
}

pub enum IntCode {
    Zero,
    Pos(Box<UIntCode>),
    Neg(Box<UIntCode>),
}

impl fmt::Display for UIntCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UIntCode::Term => write!(f, "UTerm"),
            UIntCode::Zero(ref inner) => write!(f, "UInt<{}, B0>", inner),
            UIntCode::One(ref inner) => write!(f, "UInt<{}, B1>", inner),
        }
    }
}

impl fmt::Display for IntCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntCode::Zero => write!(f, "Z0"),
            IntCode::Pos(ref inner) => write!(f, "PInt<{}>", inner),
            IntCode::Neg(ref inner) => write!(f, "NInt<{}>", inner),
        }
    }
}

pub fn gen_uint(u: u64) -> UIntCode {
    let mut result = UIntCode::Term;
    let mut x = 1u64 << 63;
    while x > u {
        x >>= 1
    }
    while x > 0 {
        result = if x & u > 0 {
            UIntCode::One(Box::new(result))
        } else {
            UIntCode::Zero(Box::new(result))
        };
        x >>= 1;
    }
    result
}

pub fn gen_int(i: i64) -> IntCode {
    if i > 0 {
        IntCode::Pos(Box::new(gen_uint(i as u64)))
    } else if i < 0 {
        IntCode::Neg(Box::new(gen_uint(i.abs() as u64)))
    } else {
        IntCode::Zero
    }
}

#[cfg_attr(feature="no_std", deprecated(
    since="1.3.0",
    note="the `no_std` flag is no longer necessary and will be removed in the future"))]
pub fn no_std() {}

// fixme: get a warning when testing without this
#[allow(dead_code)]
fn main() {
    // If you change this, change also the comments in src/consts.rs
    let highest: u64 = 1024;


    let first2: u32 = (highest as f64).log(2.0) as u32 + 1;
    let first10: u32 = (highest as f64).log(10.0) as u32 + 1;
    let uints = (0..(highest + 1))
        .chain((first2..64).map(|i| 2u64.pow(i)))
        .chain((first10..20).map(|i| 10u64.pow(i)));


    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("consts.rs");

    let mut f = File::create(&dest).unwrap();

    no_std();

    // Header stuff here!
    f.write(b"
use uint::{UInt, UTerm};
use int::{PInt, NInt};

pub use bit::{B0, B1};
pub use int::Z0; // re-export for convenience.
")
        .unwrap();

    for u in uints {
        write!(f, "pub type U{} = {};\n", u, gen_uint(u)).unwrap();
        if u <= ::std::i64::MAX as u64 && u != 0 {
            let i = u as i64;
            write!(f, "pub type P{} = {};\n", i, gen_int(i)).unwrap();
            write!(f, "pub type N{} = {};\n", i, gen_int(-i)).unwrap();
        }
    }

    tests::build_tests().unwrap();
}
