use proconio::*;

fn main() {
    input! {a: char, b: char}

    let mut b = b as u8;

    if a == 'D' {
        b ^= b'H' ^ b'D';
    }

    println!("{}", b as char);
}
