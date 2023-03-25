#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut dp = vec![std::usize::MAX; m + 1];
    dp[0] = 0;
    let mut last = vec![-1i32; m + 1];
    for i in 0..n {
        for j in (0..=m).rev() {
            if dp[j] == std::usize::MAX {
                continue;
            }

            let to = j + a[i];
            if to > m {
                continue;
            }

            let new = if last[j]+1 < i as i32 {
                dp[j] + 1
            } else {
                dp[j]
            };

            if dp[to] >= new {
                dp[to] = new;
                last[to] = i as i32;
            }
        }
    }

    for i in 1..=m {
        let mut res = dp[i];
        if res == std::usize::MAX {
            println!("-1");
            continue;
        }

        if last[i] != n as i32 - 1 {
            res += 1;
        }

        println!("{}", res);
    }
}
