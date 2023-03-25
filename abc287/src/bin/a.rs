#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [String; n]}

    let c = s.iter().filter(|s| **s == "For").count();
    if c > n / 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
