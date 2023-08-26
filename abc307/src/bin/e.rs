use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, m: u32}

    let mut dp = vec![vec![Modint::zero(); 2]; n];
    dp[0][1] = Modint::raw(m);
    for i in 0..n - 1 {
        let nd = dp[i][0] * Modint::raw(m - 2) + dp[i][1] * Modint::raw(m - 1);
        dp[i + 1][0] += nd;

        let new = dp[i][0];
        dp[i + 1][1] += new;
    }

    println!("{}", dp[n - 1][0]);
}
