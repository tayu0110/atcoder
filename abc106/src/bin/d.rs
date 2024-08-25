#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    source::line::LineSource,
    *,
};

#[fastout]
fn main() {
    input! {n: usize, m: usize, q: usize, p:[(usize, usize); m], q: [(usize, usize); q]}

    let mut memo = vec![vec![0; n + 10]; n + 10];
    for (l, r) in p {
        memo[r][l] += 1;
    }
    for v in memo.iter_mut() {
        for i in 1..v.len() {
            v[i] += v[i - 1];
        }
    }

    let mut res = vec![vec![0; n + 10]; n + 10];
    for (i, res) in res.iter_mut().enumerate().skip(1).take(n) {
        for (j, res) in res.iter_mut().take(n + 1).enumerate().skip(i) {
            let mut sum = 0;
            for (k, memo) in memo.iter().take(j + 1).enumerate().skip(i) {
                sum += memo[k] - memo[i - 1];
            }
            *res = sum;
        }
    }

    for (p, q) in q {
        println!("{}", res[p][q]);
    }
}
