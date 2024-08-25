#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: i64}

    if x < 10 {
        println!("{}", x);
        return;
    }

    let mut res = std::i64::MAX;
    for i in 1..10_i64 {
        for j in 0..10_i64 {
            let (prev, mut now) = (i, j);
            let diff = now - prev;
            let mut n = prev * 10 + now;
            while n < x {
                now += diff;
                if !(0..10).contains(&now) {
                    break;
                }
                n = n * 10 + now;
            }

            if n >= x {
                res = std::cmp::min(res, n);
            }
        }
    }

    println!("{}", res);
}
