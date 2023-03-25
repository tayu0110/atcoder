#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Chars, a: usize, b: usize}

    s.swap(a - 1, b - 1);
    println!("{}", s.into_iter().collect::<String>())
}
