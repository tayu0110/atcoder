#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, a: [i64; n]}
    const MOD: i64 = 1_000_000_007;

    let mut cum = vec![0; n+1];
    for i in 0..n {
        cum[i+1] = (cum[i] + a[i]) % MOD;
    }

    let mut res = 0;
    for i in 0..n {
        res += (cum[n] - cum[i+1] + MOD) % MOD * a[i] % MOD;
        res %= MOD;
    }

    println!("{}", res);
}
