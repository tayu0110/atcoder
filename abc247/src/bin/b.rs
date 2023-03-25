#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, p: [(String, String); n]}

    for (i, (s, t)) in p.iter().enumerate() {
        if p.iter().enumerate().filter(|(j, _)| i != *j).any(|(_, (u, v))| s == u || s == v) && p.iter().enumerate().filter(|(j, _)| i != *j).any(|(_, (u, v))| t == u || t == v) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
