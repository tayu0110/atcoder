use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, a: usize, b: usize, p: u32, q: u32}

    let mut dp = vec![vec![Modint::zero(); n + 1]; n + 1];
    let mut res = Modint::zero();
    dp[a][b] = Modint::one();

    let inv = Modint::raw(p * q).inv();
    while dp.iter().flatten().any(|v| *v != Modint::zero()) {
        let mut new = vec![vec![Modint::zero(); n + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=n {
                if dp[i][j] == Modint::zero() {
                    continue;
                }

                let np = dp[i][j] * inv;
                for k in 1..=p {
                    for l in 1..=q {
                        let to_t = n.min(i + k as usize);
                        let to_a = n.min(j + l as usize);
                        new[to_t][to_a] += np;
                    }
                }
            }
        }

        dp = new;
        for i in 1..=n {
            res += dp[n][i];
            dp[n][i] = Modint::zero();
            dp[i][n] = Modint::zero();
        }
    }

    println!("{}", res)
}
