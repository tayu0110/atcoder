#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize, c: usize}

    let d = (a + c - 1) / c * c;
    if d <= b {
        println!("{}", d);
    } else {
        println!("-1");
    }
}
