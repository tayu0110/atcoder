#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}
    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b);
        t[b-1].push(a);
    }

    for v in t.iter_mut() {
        v.sort();
        print!("{} ", v.len());
        println!("{}", v.iter().join(" "));
    }
}
