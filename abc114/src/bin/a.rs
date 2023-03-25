#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: usize}
    if x == 3 || x == 5 || x == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
