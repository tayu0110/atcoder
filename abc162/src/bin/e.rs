use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: u32, k: usize}

    let mut memo = vec![Modint::one(); k + 1];
    for i in (1..=k).rev() {
        let m = k / i;
        memo[i] = Modint::raw(m as u32).pow(n as u64);
        for j in (2..).take_while(|&j| i * j <= k) {
            memo[i] = memo[i] - memo[i * j];
        }
    }

    println!(
        "{}",
        memo.iter()
            .enumerate()
            .fold(Modint::zero(), |s, (i, &v)| s + v * Modint::raw(i as u32))
    )
}
