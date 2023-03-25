#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut n: usize, k: usize}

    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = format!("{}200", n).parse().unwrap();
        }
    }

    println!("{}", n);
}
