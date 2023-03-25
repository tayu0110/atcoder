#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {mut s: Chars}
    s.sort();
    let s = s.into_iter().collect::<String>();
    println!("{}", s);
}
