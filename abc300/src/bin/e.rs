use math::factorize;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: u64}

    let fact = factorize(n);
    let mut t = vec![0; 6];
    for &f in &fact {
        if f != 2 && f != 3 && f != 5 {
            println!("0");
            return;
        }

        t[f as usize] += 1;
    }

    let (s, t, u) = (t[2], t[3], t[5]);
    let mut dp = vec![vec![vec![Modint::zero(); u + 1]; t + 1]; s + 1];
    dp[s][t][u] = Modint::one();

    let p = Modint::one() / Modint::raw(5);
    let mut res = Modint::zero();
    for _ in 0..200 {
        let mut new = vec![vec![vec![Modint::zero(); u + 1]; t + 1]; s + 1];
        for i in 0..=s {
            for j in 0..=t {
                for k in 0..=u {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    let next = dp[i][j][k] * p;
                    if i >= 1 {
                        new[i - 1][j][k] += next;
                    }
                    if j >= 1 {
                        new[i][j - 1][k] += next;
                    }
                    if i >= 2 {
                        new[i - 2][j][k] += next;
                    }
                    if k >= 1 {
                        new[i][j][k - 1] += next;
                    }
                    if i >= 1 && j >= 1 {
                        new[i - 1][j - 1][k] += next;
                    }
                }
            }
        }
        res += new[0][0][0];
        new[0][0][0] = Modint::zero();
        dp = new;
    }

    println!("{}", res)
}
