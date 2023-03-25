#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, s: [String; n]}

    let set = s.into_iter().collect::<std::collections::HashSet<_>>();
    println!("{}", set.len());
}
