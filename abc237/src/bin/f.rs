#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize}
    const MOD: usize = 998244353;

    let mut dp = vec![vec![vec![vec![0usize; m+1]; m+1]; m+1]; n+1];
    dp[0][0][0][0] = 1;
    for i in 0..n {
        for mut j in 0..=m {
            for mut k in 0..=m {
                for mut l in 0..=m {
                    for p in 1..=m {
                        if j == 0 { j = std::usize::MAX; }
                        if k == 0 { k = std::usize::MAX; }
                        if l == 0 { l = std::usize::MAX; }
                        let mut lis = [j, k, l, std::usize::MAX];
                        if lis[0] > lis[1] || lis[1] > lis[2] || lis[2] > lis[3] {
                            continue;
                        }
                        for i in 0..4 {
                            if p <= lis[i] {
                                lis[i] = p;
                                break;
                            }
                        }
                        if lis[3] != std::usize::MAX {
                            continue;
                        }
                        let (mut nj, mut nk, mut nl) = (lis[0], lis[1], lis[2]);
                        if nj == std::usize::MAX { nj = 0; }
                        if nk == std::usize::MAX { nk = 0; }
                        if nl == std::usize::MAX { nl = 0; }
                        if j == std::usize::MAX { j = 0; }
                        if k == std::usize::MAX { k = 0; }
                        if l == std::usize::MAX { l = 0; }
                        dp[i+1][nj][nk][nl] += dp[i][j][k][l];
                        dp[i+1][nj][nk][nl] %= MOD;
                    }
                }
            }
        }
    }

    let mut res = 0;
    for j in 1..=m {
        for k in 1..=m {
            for l in 1..=m {
                res += dp[n][j][k][l];
                res %= MOD;
            }
        }
    }

    println!("{}", res);
}
