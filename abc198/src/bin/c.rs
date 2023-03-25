#![allow(unused_imports)]
use num::integer::Roots;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {r: u128, x: u128, y: u128}

    let sq = (x * x + y * y).sqrt();
    if r > sq {
        println!("2");
        return;
    }
    if sq * sq == (x * x + y * y) {
        println!("{}", (sq + r - 1) / r);
    } else {
        let res = (sq + r - 1) / r;
        if (res * r) * (res * r) >= (x * x + y * y) {
            println!("{}", res);
        } else {
            println!("{}", res + 1);
        }
    }
}
