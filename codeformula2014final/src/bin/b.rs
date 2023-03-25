#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    if n % 2 == 0 {
        println!("{}", n / 2)
    } else {
        println!("{}", n / 2 + 3)
    }
}
