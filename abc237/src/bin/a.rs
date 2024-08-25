#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: i64}

    if (-(1 << 31)..(1 << 31)).contains(&n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
