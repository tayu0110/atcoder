use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;
const MAX: usize = 1 << 11;
const MASK: usize = MAX - 1;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut dp = vec![vec![Modint::zero(); MAX]; n + 1];
    dp[0][1] = Modint::one();

    for (i, a) in a.into_iter().enumerate() {
        let inv = Modint::raw(a as u32).inv();
        for j in 0..MAX {
            let now = dp[i][j];
            for k in 1..=10.min(a) {
                let next = (j | (j << k)) & MASK;
                dp[i + 1][next] += now * inv;
            }

            if a > 10 {
                dp[i + 1][j] += now * inv * Modint::raw(a as u32 - 10);
            }
        }
    }

    println!(
        "{}",
        (0..MAX)
            .filter(|&i| i & (1 << 10) != 0)
            .map(|i| dp[n][i])
            .fold(Modint::zero(), |s, v| s + v)
    )
}
