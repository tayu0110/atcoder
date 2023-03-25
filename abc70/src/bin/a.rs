#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: u16}

    if n % 10 == n / 100 {
        println!("Yes")
    } else {
        println!("No")
    }
}
