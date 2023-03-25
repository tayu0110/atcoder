#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut a: usize, mut b: usize}
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    if a * 2 == b || a * 2 + 1 == b {
        println!("Yes")
    } else {
        println!("No")
    }
}
