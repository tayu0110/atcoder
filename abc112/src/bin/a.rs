#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    if n == 1 {
        println!("Hello World")
    } else {
        input! {a: u8, b: u8}
        println!("{}", a + b)
    }
}
