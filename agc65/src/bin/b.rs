use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, _p: [usize; n]}

    let mut dp = vec![vec![Modint::zero(); n + 1]; n];
    dp[n - 1][n] = Modint::one();
    for i in (1..n).rev() {
        for j in 0..=n {
            if dp[i][j] == Modint::zero() {
                continue;
            }

            for k in 0..=j.min(n - 1) {
                dp[i - 1][k] = dp[i - 1][k] + dp[i][j];
            }
        }
    }

    eprintln!("{dp:?}");
    println!("{}", dp[0][1] + dp[0][0]);
}
