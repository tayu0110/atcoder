#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, a: [Chars; 2*n]}
    let mut na = vec![vec![0; m]; 2*n];
    for i in 0..n*2 {
        for j in 0..m {
            if a[i][j] == 'G' {
                na[i][j] = 0;
            } else if a[i][j] == 'C' {
                na[i][j] = 1;
            } else {
                na[i][j] = 2;
            }
        }
    }

    let mut res = (0..n*2).collect::<Vec<_>>();
    let mut win = vec![0; n*2];

    for i in 0..m {
        for j in (0..n*2).step_by(2) {
            let result = (na[res[j]][i] + 3 - na[res[j+1]][i]) % 3;
            if result == 2 {
                win[res[j]] += 1;
            } else if result == 1 {
                win[res[j+1]] += 1;
            }
        }

        res.sort_by_key(|c| std::cmp::Reverse((win[*c], n*2 - *c)));
    }

    for v in res {
        println!("{}", v+1);
    }
}
