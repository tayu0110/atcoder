use itertools::Itertools;
use proconio::input;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize}

    let mut dp = vec![vec![Modint::zero(); n]; n + 1];
    dp[1][0] = Modint::one();

    let inv2 = Modint::raw(2).inv();
    for i in 2..n + 1 {
        // i人いるときに先頭を排除する確率 p(i,0) = 2^{i-1}/(2^{i}-1), p(i,j) = 2^{i-j-1}/(2^{i}-1) = p(i,0)(1/2^{j})
        let p0 = Modint::raw(2).pow(i as u64 - 1) / (Modint::raw(2).pow(i as u64) - Modint::one());

        // 先頭からの累積和 cum[j] = dp[i-1][0]p(i,j-1) + dp[i-1][1]p(i,j-2) + ... + dp[i-1][j]p(i,0)
        let mut cum = vec![Modint::zero(); i - 1];
        cum[0] = dp[i - 1][0] * p0;
        for j in 1..i - 1 {
            cum[j] = cum[j - 1] * inv2 + dp[i - 1][j] * p0;
        }

        // 後ろからの累積和 rcum[j] = dp[i-1][i-2]p(i,0) + dp[i-1][i-3]p(i,1) + ... + dp[i-1][j]p(i,i-1-j)
        let mut rcum = vec![Modint::zero(); i - 1];
        rcum[i - 2] = dp[i - 1][i - 2] * p0;
        let mut now = p0;
        for j in (0..i - 2).rev() {
            now *= inv2;
            rcum[j] = rcum[j + 1] + dp[i - 1][j] * now;
        }

        let mut now = inv2;
        for j in 0..i {
            if j < i - 1 {
                dp[i][j] = rcum[j] * now;
            }
            if j > 0 {
                dp[i][j] += cum[j - 1];
            }
            now *= inv2;
        }
    }

    println!("{}", dp[n].iter().join(" "));
}
