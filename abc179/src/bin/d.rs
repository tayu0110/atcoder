use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); k]}

    let mut dp = vec![Modint::zero(); n].into_boxed_slice();
    dp[0] = Modint::one();
    dp[1] -= Modint::one();

    for i in 0..n {
        if i > 0 {
            dp[i] += dp[i - 1];
        }
        for &(l, r) in &p {
            if i + l < n {
                dp[i + l] += dp[i];
            }
            if i + r + 1 < n {
                dp[i + r + 1] -= dp[i];
            }
        }
    }

    println!("{}", dp[n - 1]);
}
