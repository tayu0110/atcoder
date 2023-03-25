#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: u16}
    if n < 1000 {
        println!("ABC")
    } else {
        println!("ABD")
    }
}
