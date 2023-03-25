#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize}

    let diff = b - a;
    println!("{}", (1..=diff).sum::<usize>() - b);
}
