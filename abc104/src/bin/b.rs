#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {s: Chars}

    if s[0] != 'A' {
        println!("WA");
        std::process::exit(0);
    }

    if s[2..s.len()-1].iter().filter(|c| **c == 'C').count() != 1 {
        println!("WA");
        std::process::exit(0);
    }

    if s.iter().filter(|c| c.is_ascii_uppercase()).count() != 2 {
        println!("WA");
        std::process::exit(0);
    }

    println!("AC");
}
