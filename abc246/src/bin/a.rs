#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32}
    println!("{} {}", x1 ^ x2 ^ x3, y1 ^ y2 ^ y3);
}
