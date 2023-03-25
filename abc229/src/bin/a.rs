#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: String, t: String}

    if s == "#." && t == ".#" {
        println!("No");
    } else if s == ".#" && t == "#." {
        println!("No");
    } else {
        println!("Yes");
    }
}
