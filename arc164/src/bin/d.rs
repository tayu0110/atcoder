use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, t: marker::Chars}

    let mut dp = vec![vec![Modint::zero(); n + 1]; n + 1];
    let mut cnt = vec![vec![Modint::zero(); n + 1]; n + 1];
    cnt[0][0] = Modint::one();

    for k in 0..2 * n {
        for p in 0..=k {
            let m = k - p;
            if p > n || m > n {
                continue;
            }
            let rem = p.max(m) - p.min(m);
            let new = dp[p][m];
            let nc = cnt[p][m];
            if t[k] != '-' && p < n {
                dp[p + 1][m] += new;
                cnt[p + 1][m] += nc;
                if p >= m {
                    dp[p + 1][m] += Modint::raw(rem as u32 * 2 + 1) * nc;
                }
            }
            if t[k] != '+' && m < n {
                dp[p][m + 1] += new;
                cnt[p][m + 1] += nc;
                if m >= p {
                    dp[p][m + 1] += Modint::raw(rem as u32 * 2 + 1) * nc;
                }
            }
        }
    }

    println!("{}", dp[n][n]);
}
