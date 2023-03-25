#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, s: [Chars; n]}

    // 0: win, 1: lose, 2: not decided
    let mut dp = vec![vec![2; n]; 1 << n];
    for i in 0..n {
        dp[(1 << n) - 1][i] = 1;
    }

    for i in (0..(1 << n)).rev() {
        for j in 0..n {
            if i & (1 << j) != 0 {
                continue;
            }

            let ni = i | (1 << j);
            let lc = *s[j].last().unwrap();
            let mut result = 0;
            for next in 0..n {
                if ni & (1 << next) != 0 {
                    continue;
                }

                if s[next][0] != lc {
                    continue;
                }

                if dp[ni][next] == 0 {
                    result = 1;
                }
            }
            dp[i][j] = result;
        }
    }

    for i in 0..n {
        if dp[0][i] == 0 {
            println!("First");
            return;
        }
    }

    println!("Second");
}
