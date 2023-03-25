#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {h: usize, a: usize}
    println!("{}", (h+a-1) / a);
}
