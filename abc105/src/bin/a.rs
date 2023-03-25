#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: u8, k: u8}

    println!("{}", (n % k != 0) as u8)
}
