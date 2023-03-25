#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: usize, x: usize, y: usize}

    if a >= n {
        println!("{}", x * n);
    } else {
        println!("{}", x * a + (n - a) * y);
    }
}
