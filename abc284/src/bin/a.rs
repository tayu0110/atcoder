#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [String; n]}

    for s in s.into_iter().rev() {
        println!("{}", s);
    }
}
