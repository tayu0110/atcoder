#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    const MOD: usize = 998244353;
    input! {n: usize, mut a: [usize; n]}

    let mut dp = vec![0; 10];
    dp[a[0]] += 1;
    a.remove(0);

    for a in a {
        let mut tmp = vec![0; 10];

        for i in 0..10 {
            tmp[(i + a) % 10] += dp[i];
            tmp[(i + a) % 10] %= MOD;
            tmp[(i * a) % 10] += dp[i];
            tmp[(i * a) % 10] %= MOD;
        }

        std::mem::swap(&mut tmp, &mut dp);
    }

    for res in dp {
        println!("{}", res);
    }
}
