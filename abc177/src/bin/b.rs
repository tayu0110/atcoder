#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: Chars, t: Chars}
    let sl = s.len();
    let tl = t.len();

    let mut res = std::usize::MAX;
    for i in 0..sl-tl+1 {
        let mut sum = 0;
        for j in 0..tl {
            if s[i+j] != t[j] {
                sum += 1;
            }
        }

        res = std::cmp::min(res, sum);
    }

    println!("{}", res);
}
