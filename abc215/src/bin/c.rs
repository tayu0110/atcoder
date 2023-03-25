#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {mut s: Chars, k: usize}

    
    s.sort();
    let mut s = s.iter().permutations(s.len()).collect::<Vec<_>>();
    s.sort();
    s.dedup();
    println!("{}", s[k-1].iter().map(|c| **c).collect::<String>());
}
