#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {a: usize, b: usize}
    let a = a + b;

    if a >= 15 && b >= 8 {
        println!("1");
    } else if a >= 10 && b >= 3 {
        println!("2");
    } else if a >= 3 {
        println!("3");
    } else {
        println!("4");
    }
}
