#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {mut a: usize, mut b: usize, mut c: usize, mut x: usize, mut y: usize}
    c = std::cmp::min(a+b, c*2);
    if x > y {
        std::mem::swap(&mut x, &mut y);
        std::mem::swap(&mut a, &mut b);
    }

    let res = x * c;
    y -= x;
    b = std::cmp::min(b, c);
    println!("{}", res + y * b);
}
