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
        if u <= std::i64::MAX as u64 && u != 0 {
            let i = u as i64;
            f.write(format!("pub type P{} = {};\n", i, gen_int(i)).as_bytes()).unwrap();
            f.write(format!("pub type N{} = {};\n", i, gen_int(-i)).as_bytes()).unwrap();
        }
    }
}
