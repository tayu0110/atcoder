#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    for i in (0..=n).rev() {
        println!("{}", i);
    }
}
