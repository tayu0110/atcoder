#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize}

    let mut res = 0;
    for i in (1..=n).step_by(2) {
        let mut num = 0;
        for j in 1..=i {
            if i % j == 0 {
                num += 1;
            }
        }

        if num == 8 {
            res += 1;
        }
    }

    println!("{}", res);
}
