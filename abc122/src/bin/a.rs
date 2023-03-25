#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {b: char}

    if b == 'A' || b == 'T' {
        println!("{}", (b as u8 ^ b'A' ^ b'T') as char);
    } else {
        println!("{}", (b as u8 ^ b'C' ^ b'G') as char);
    }
}
