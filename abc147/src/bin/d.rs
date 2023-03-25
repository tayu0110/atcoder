#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    const MOD: usize = 1000_000_007;
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    for i in 0..=60 {
        let mut o = 0;
        let mut z = 0;
        for &na in &a {
            if na &(1 << i) != 0 {
                o += 1;
            } else {
                z += 1;
            }
        }

        let t = o * z % MOD;
        let p = (1 << i) % MOD;
        res += t * p % MOD;
        res %= MOD;
    }

    println!("{}", res);
}
