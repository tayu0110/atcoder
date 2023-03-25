#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, k: usize, mut h: [usize; n]}
    h.sort();
    println!("{}", h.iter().take(n.saturating_sub(k)).sum::<usize>());
}
