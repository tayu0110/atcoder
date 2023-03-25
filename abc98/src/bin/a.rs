#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: i32, b: i32}

    println!("{}", std::cmp::max(a + b, std::cmp::max(a - b, a * b)))
}
