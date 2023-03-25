#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    for (a, b) in p {
        println!("{}", a * b);
    }
}
