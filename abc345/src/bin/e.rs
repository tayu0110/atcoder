use std::collections::VecDeque;

use proconio::*;

const K: usize = 510;

fn main() {
    input! {n: usize, k: usize, mut p: [(u32, i64); n]}
    p.insert(0, (0, 0));
    p.push((0, 0));
    let mut diff = VecDeque::new();
    for _ in 0..K {
        diff.push_back(vec![[(-1, u32::MAX); 2]; k + 1]);
    }
    diff[K - 1][0] = [(0, u32::MAX), (-1, u32::MAX)];
    let mut dp = vec![vec![-1; k + 1]; n + 2];
    for i in 1..=n + 1 {
        let (c, v) = p[i];
        diff.push_back(vec![[(-1, u32::MAX); 2]; k + 1]);
        for j in 0..=k {
            let d = K - 1 - j;
            if diff[d].len() <= j {
                continue;
            }
            let res = diff[d][j];
            let t = if res[0].1 == c {
                if res[1].0 < 0 {
                    (-1, u32::MAX)
                } else {
                    dp[i][j] = v + res[1].0;
                    (v + res[1].0, c)
                }
            } else {
                if res[0].0 < 0 {
                    (-1, u32::MAX)
                } else {
                    dp[i][j] = v + res[0].0;
                    (v + res[0].0, c)
                }
            };
            diff[K - j][j] = [t, (-1, u32::MAX)];
            if j > 0 {
                let [l0, l1] = diff[K - j][j - 1];
                let [r0, r1] = diff[K - j][j];
                diff[K - j][j] = if l0 > r0 {
                    if r0 > l1 && r0.1 != l0.1 {
                        [l0, r0]
                    } else if r1 > l1 && r1.1 != l0.1 {
                        [l0, r1]
                    } else {
                        [l0, l1]
                    }
                } else {
                    if l0 > r1 && l0.1 != r0.1 {
                        [r0, l0]
                    } else if l1 > r1 && l1.1 != r0.1 {
                        [r0, l1]
                    } else {
                        [r0, r1]
                    }
                };
            }
        }
        diff.pop_front();
    }

    println!("{}", dp[n + 1][k])
}
