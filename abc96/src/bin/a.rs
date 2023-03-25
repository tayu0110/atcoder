#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize}

    if b < a {
        println!("{}", a - 1)
    } else {
        println!("{}", a);
    }
}
