#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Chars}
    s.iter_mut()
        .for_each(|c| *c = if *c == '0' { '1' } else { '0' });
    println!("{}", s.iter().collect::<String>())
}
