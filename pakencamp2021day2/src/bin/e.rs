use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: u64, m: usize}

    let m = m % M;
    println!(
        "{}",
        m * m.saturating_sub(1) % M * m.saturating_sub(2).pow_mod(n - 2, M) % M
    );
}
