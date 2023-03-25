use proconio::*;

const MOD: u128 = 1000_000_007;

fn main() {
    input! {l: u128}

    let (s, t, u, v) = ((l - 1) / 4, (l - 2) / 3, (l + 1) / 3, (l - 2) / 2);
    println!(
        "{}",
        ((s - 1) * s / 2 % MOD
            + t.saturating_sub(s) * (2 * l).saturating_sub(3 * s + 3 * t + 7) / 2 % MOD
            + u.saturating_sub(2) * u.saturating_sub(3) / 2 % MOD
            + v.saturating_sub(u) * (l.saturating_sub(u + v + 3)) % MOD)
            % MOD
    );
}
