#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, s: [usize; n], t: [usize; n], u: [usize; n], v: [usize; n]}

    let mut res = vec![vec![0; n]; n];

    for i in 0..n {
        if s[i] == 0 {
            for j in 0..n {
                res[i][j] |= u[i];
            }
        }
        if t[i] == 0 {
            for j in 0..n {
                res[j][i] |= v[i];
            }
        }
    }

    
}
