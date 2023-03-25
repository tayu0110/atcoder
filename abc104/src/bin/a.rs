#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {r: usize}

    if r < 1200 {
        println!("ABC")
    } else if r < 2800 {
        println!("ARC")
    } else {
        println!("AGC")
    }
}
