use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: u32}

    let mut dp = vec![vec![Modint::zero(); 2]; n as usize + 1];
    dp[0][1] = Modint::one();

    let invn = Modint::raw(n).inv();
    for i in 2..n as usize + 1 {
        let n = Modint::new(n as u64);
        let t = (Modint::one() - ((Modint::raw(i as u32) - Modint::one()) * invn).pow(2)).inv();
        for j in 0..2 {
            // first
            {
                // dp[i][j] += dp[i - 1][j ^ 1] * (n - Modint::raw(i as u32) + Modint::one()) * invn * t;
            }
            // second
            {
                // dp[i][j] += dp[i - 1][j]
                //     * (n - Modint::raw(i as u32) + Modint::one())
                //     * invn
                //     * t
                //     * (Modint::raw(i as u32) - Modint::one())
                //     * invn;
            }
        }
    }

    let mut f = Modint::zero();
    let mut b = Modint::zero();
    for i in 1..=n as usize {
        f += dp[i][0];
        b += dp[i][1];
    }

    println!("{} {}", f.val(), b.val())
}
