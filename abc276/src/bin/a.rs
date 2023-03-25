#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {mut s: Chars}

    s.reverse();
    for i in 0..s.len() {
        if s[i] == 'a' {
            println!("{}", s.len() - i);
            return;
        }
    }

    println!("-1");
}
