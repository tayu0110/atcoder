#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, a: [usize; n]}
        println!("{}", a.into_iter().map(|v| v % 2).sum::<usize>())
    }
}
