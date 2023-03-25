#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], q: [(usize, usize); m]}

    let s = {
        let mut t = vec![vec![false; n]; n];
        for (a, b) in p {
            t[a - 1][b - 1] = true;
            t[b - 1][a - 1] = true;
        }
        t
    };
    let t = {
        let mut t = vec![vec![false; n]; n];
        for (a, b) in q {
            t[a - 1][b - 1] = true;
            t[b - 1][a - 1] = true;
        }
        t
    };

    'base: for v in (0..n).permutations(n) {
        for i in 0..n {
            for j in 0..n {
                if s[i][j] != t[v[i]][v[j]] {
                    continue 'base;
                }
            }
        }

        println!("Yes");
        return;
    }

    println!("No")
}
