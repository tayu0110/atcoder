#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    if 1 <= n && n <= 125 {
        println!("4")
    } else if n <= 211 {
        println!("6")
    } else {
        println!("8")
    }
}
