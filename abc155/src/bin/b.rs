#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n]}

    if a.into_iter()
        .filter(|&v| v % 2 == 0)
        .all(|v| v % 3 == 0 || v % 5 == 0)
    {
        println!("APPROVED")
    } else {
        println!("DENIED")
    }
}
