#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {x: usize}

    if x % 100 == 0 && x > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
