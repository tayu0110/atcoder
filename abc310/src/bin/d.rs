use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, t: usize, m: usize, p: [(usize, usize); m]}

    let set = p
        .into_iter()
        .map(|(a, b)| ((a - 1).min(b - 1), (a - 1).max(b - 1)))
        .collect::<HashSet<_>>();
    let mut dp = vec![vec![0; 1 << n]; t];
    let mut ok = vec![false; 1 << n];
    for i in 0..1 << n {
        let mut bad = false;
        for j in 0..n {
            for k in j + 1..n {
                if i & (1 << j) != 0 && i & (1 << k) != 0 && set.contains(&(j, k)) {
                    bad = true;
                }
            }
        }

        ok[i] = !bad;
        if !bad && i & 1 != 0 {
            dp[0][i] = 1;
        }
    }

    for i in 0..t - 1 {
        for j in 1..1 << n {
            if dp[i][j] == 0 {
                continue;
            }
            let rem = ((1 << n) - 1) ^ j;
            if rem == 0 {
                continue;
            }
            let cl = rem & rem.wrapping_neg();
            let mut now = rem;
            while now > 0 {
                if ok[now] && now & cl != 0 {
                    dp[i + 1][j | now] += dp[i][j];
                }
                now = (now - 1) & rem;
            }
        }
    }

    println!("{}", dp[t - 1][(1 << n) - 1])
}
