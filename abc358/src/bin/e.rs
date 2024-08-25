use proconio::*;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {k: usize, c: [usize; 26]}

    let com = combination(10000);
    let mut dp = vec![Modint::zero(); k + 1];
    dp[0] = Modint::one();
    for c in c {
        let mut new = vec![Modint::zero(); k + 1];
        for i in 0..=k {
            for j in 0..=i.min(c) {
                new[i] += dp[i - j] * com(i as u32, j as u32);
            }
        }

        dp = new;
    }

    println!(
        "{}",
        dp[1..=k].iter().copied().fold(Modint::zero(), |s, v| s + v)
    );
}
