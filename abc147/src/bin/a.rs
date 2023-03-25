#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {a: usize, b: usize, c: usize}
    if a+b+c >= 22 {
        println!("bust");
    } else {
        println!("win")
    }
}
