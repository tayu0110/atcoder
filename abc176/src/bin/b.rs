#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: Bytes};
    if n.into_iter().map(|c| c - b'0').fold(0, |s, v| (s + v) % 9) == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
