use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, c: [marker::Bytes; n]}

    let mut res = vec![vec![-2; n]; n];
    let mut nt = VecDeque::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                res[i][j] = 0;
                nt.push_front((i, j));
            } else if c[i][j] != b'-' {
                res[i][j] = 1;
                nt.push_back((i, j));
            }
        }
    }

    while let Some((src, dst)) = nt.pop_front() {
        for prev in 0..n {
            if c[prev][src] == b'-' {
                continue;
            }

            for next in 0..n {
                if c[dst][next] != c[prev][src] {
                    continue;
                }

                if res[prev][next] >= -1 {
                    continue;
                }

                res[prev][next] = res[src][dst] + 2;
                nt.push_back((prev, next));
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if res[i][j] == -2 {
                res[i][j] = -1;
            }
        }
    }

    for res in res {
        println!("{}", res.iter().join(" "))
    }
}
