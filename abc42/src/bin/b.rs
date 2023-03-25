#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, _l: usize, mut s: [String; n]}
    s.sort();

    println!("{}", s.iter().join(""));
}
