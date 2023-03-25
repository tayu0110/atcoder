#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: usize, w: usize}

    if s > w {
        println!("safe");
    } else {
        println!("unsafe");
    }
}
