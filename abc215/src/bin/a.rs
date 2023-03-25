#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: String}

    if s == "Hello,World!" {
        println!("AC");
    } else {
        println!("WA");
    }
}
