use proconio::*;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {n: usize, m: usize, mut x: [i64; n], mut y: [i64; m]}

    x.sort();
    y.sort();
    for i in (0..n).rev() {
        x[i] += x[0];
    }
    for i in (0..m).rev() {
        y[i] += y[0];
    }

    let mut sum = 0;
    for (i, v) in y.windows(2).enumerate() {
        sum += (i + 1) as i64 * (m - i - 1) as i64 % MOD * (v[1] - v[0]) % MOD;
        sum %= MOD;
    }
    let mut res = 0;
    for (i, v) in x.windows(2).enumerate() {
        res += (i + 1) as i64 * (n - i - 1) as i64 % MOD * (v[1] - v[0]) % MOD * sum % MOD;
        res %= MOD;
    }

    println!("{}", res);
}
