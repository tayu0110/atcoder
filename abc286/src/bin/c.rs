#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: usize, b: usize, s: Chars}

    let mut res = std::usize::MAX;
    for i in 0..n {
        let mut t = [&s[i..n], &s[0..i]].concat();
        let mut cost = i * a;

        for i in 0..n {
            if t[i] == t[n - i - 1] {
                continue;
            }

            cost += b;
            t[n - i - 1] = t[i];
        }

        res = std::cmp::min(res, cost);
    }

    println!("{}", res);
}
