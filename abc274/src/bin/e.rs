#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn speed(s: usize, n: usize) -> usize {
    // eprintln!("s: {:b}, (1 << n) - 1: {:b}", s, !((1 << n)-1));
    1 << (s & !((1 << n) - 1)).count_ones()
}

#[fastout]
fn main() {
    input! {n: usize, m: usize, p: [(i64, i64); n], q: [(i64, i64); m]}

    let mut dp = vec![vec![std::f64::MAX / 2f64; 1 << (n+m)]; n+m];
    for i in 0..n+m {
        let (x, y) = if i >= n {
            q[i-n]
        } else {
            p[i]
        };

        dp[i][1 << i] = ((x * x + y * y) as f64).sqrt();
    }

    for i in 1..(1 << (n+m)) {
        for j in 0..n+m {
            if dp[j][i] > std::f64::MAX / 4f64 {
                continue;
            }

            for k in 0..n+m {
                if i & (1 << k) != 0 {
                    continue;
                }

                let s = speed(i, n);
                // eprintln!("i: {:b}, s: {}", i, s);
                let (x, y) = if j >= n {
                    q[j-n]
                } else {
                    p[j]
                };
                let (tx, ty) = if k >= n {
                    q[k-n]
                } else {
                    p[k]
                };
                let dist = (((x-tx)*(x-tx) + (y-ty)*(y-ty)) as f64).sqrt();

                if dp[j][i] + dist / (s as f64) < dp[k][i | (1 << k)] {
                    dp[k][i | (1 << k)] = dp[j][i] + (dist / s as f64);
                }
            }
        }
    }

    let mut res = std::f64::MAX / 2f64;
    for i in 0..(1 << (n+m)) {
        if i & ((1 << n)-1) != ((1 << n)-1) {
            continue;
        }

        for j in 0..n+m {
            if dp[j][i] > std::f64::MAX / 4f64 {
                continue;
            }

            let (x, y) = if j >= n {
                q[j-n]
            } else {
                p[j]
            };
            let s = speed(i, n);
            let dist = ((x * x + y * y) as f64).sqrt();

            if res > dp[j][i] + (dist / s as f64) {
                res = dp[j][i] + dist / s as f64;
            }
        }
    }

    println!("{}", res);
}
