#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut a: [usize; 3], k: usize}

    for _ in 0..k {
        a.sort();
        a[2] *= 2;
    }

    println!("{}", a.into_iter().sum::<usize>())
}
