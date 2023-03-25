#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {x: i32, a: i32, b: i32}

    if (x-a).abs() < (x-b).abs() {
        println!("A");
    } else {
        println!("B");
    }
}
