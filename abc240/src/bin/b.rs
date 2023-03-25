#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, a: [usize; n]}
    let set = a.into_iter().collect::<std::collections::HashSet<_>>();
    println!("{}", set.len());
}
