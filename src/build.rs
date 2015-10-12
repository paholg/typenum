/// Generates a string in the format of our `UInt`s from the given number.
/// Useful for generating type aliases.
pub fn gen_uint(u: u64) -> String {
    let bitstring = format!("{:b}", u);
    let bits = bitstring.chars().skip_while(|&c| c == '0');
    let mut uint = "UTerm".into();
    for bit in bits {
        uint = format!("UInt<{}, {}>", uint, bit);
    }
    uint
}

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
    let uints = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let ints = [-5, -4, -3, -2, -1, 1, 2, 3, 4, 5];

    for &u in uints.iter() {
        println!("pub type U{} = {};", u, gen_uint(u));
    }

    println!("");
    for &i in ints.iter() {
        let sign = if i > 0 { 'P' } else if i < 0 { 'N' } else { panic!("Can't alias Z0, it's already got a nice name."); };
        // fixme: just call abs()?
        let temp = if i > 0 { i } else { -i };
        println!("pub type {}{} = {};", sign, temp, gen_int(i));
    }
}
