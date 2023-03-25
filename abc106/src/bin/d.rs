#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, q: usize, p:[(usize, usize); m], q: [(usize, usize); q]}

    let mut memo = vec![vec![0; n+10]; n+10];
    for (l, r) in p {
        memo[r][l] += 1;
    }
    for v in memo.iter_mut() {
        for i in 1..v.len() {
            v[i] += v[i-1];
        }
    }

    let mut res = vec![vec![0; n+10]; n+10];
    for i in 1..n+1 {
        for j in i..n+1 {
            let mut sum = 0;
            for k in i..=j {
                sum += memo[k][k] - memo[k][i-1];
            }
            res[i][j] = sum;
        }
    }

    for (p, q) in q {
        println!("{}", res[p][q]);
    }
}
