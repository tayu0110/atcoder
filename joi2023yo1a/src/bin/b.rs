#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}
    if n % 11 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
