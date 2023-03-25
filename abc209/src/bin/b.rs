#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, x: usize, mut a: [usize; n]}
    for i in (0..n).skip(1).step_by(2) {
        a[i] -= 1;
    }

    let sum = a.into_iter().sum::<usize>();
    if sum <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
