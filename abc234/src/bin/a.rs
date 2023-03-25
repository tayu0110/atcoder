#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {t: usize}

    let f = |x| x * x + 2 * x + 3;
    println!("{}", f(f(f(t)+t)+f(f(t))));
}
