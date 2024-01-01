use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, x: usize, t: [usize; n]}

    let mut res = Modint::zero();
    let mut dp = vec![Modint::zero(); x + 1];
    dp[0] = Modint::one();

    let inv = Modint::raw(n as u32).inv();
    for i in 0..=x {
        if dp[i] == Modint::zero() {
            continue;
        }

        for (j, &t) in t.iter().enumerate() {
            if i + t > x {
                if j == 0 {
                    res += dp[i] * inv;
                }
                continue;
            }

            dp[i + t] = dp[i + t] + dp[i] * inv;
        }
    }

    println!("{}", res)
}
