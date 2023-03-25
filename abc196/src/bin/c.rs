#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize}
    println!("{}", (1..=n).map(|c| c.to_string().chars().chain(c.to_string().chars()).collect::<String>().parse::<usize>().unwrap()).take_while(|c| *c <= n).count());
}
