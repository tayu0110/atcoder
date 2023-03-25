#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {r: usize}

    println!("{}", 3 * r * r);
}
