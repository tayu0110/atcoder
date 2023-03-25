#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}
    let n = n % 10;
    if n == 3 {
        println!("bon")
    } else if n == 0 || n == 1 || n == 6 || n == 8 {
        println!("pon")
    } else {
        println!("hon")
    }
}
