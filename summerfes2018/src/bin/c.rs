use proconio::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!(
        "{}",
        (a.into_iter().fold(1, |s, v| (s * (v + 2)) % MOD) + MOD - 2) % MOD
    )
}
