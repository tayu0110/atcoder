#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {h: usize, _w: usize, s: [Chars; h]}

    let s = s.into_iter().flatten().filter(|c| *c == '#').count();
    println!("{}", s);
}
