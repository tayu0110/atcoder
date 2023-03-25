#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {a: usize, b: usize, c: usize, x: usize}

    if x <= a {
        println!("1");
    } else if x > b {
        println!("0");
    } else {
        println!("{}", c as f64 / (b - a) as f64);
    }
}
