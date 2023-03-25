#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    let mut res = 0.0;
    for _ in 0..n {
        input! {x: f64, u: String}

        if u == "JPY" {
            res += x;
        } else {
            res += x * 380000.0;
        }
    }

    println!("{}", res);
}
