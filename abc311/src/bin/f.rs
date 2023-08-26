use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut dp = vec![vec![Modint::zero(); n + 1]; m + 1];
    dp[0][n] = Modint::one();

    for i in 0..m {
        let mut sum = dp[i].iter().fold(Modint::zero(), |s, &v| s + v);
        for j in 0..=n {
            dp[i + 1][j] = sum;
            if j == n || s[j][i] == '#' {
                break;
            }
            if j > 0 {
                sum -= dp[i][j - 1];
            }
        }
    }

    println!("{}", dp[m].iter().fold(Modint::zero(), |s, &v| s + v))
}
