#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

const MOD: usize = 1000_000_007;

fn main() {
    input! {n: usize}

    let mut map = std::collections::HashMap::new();
    for i in 1..=n {
        let mut now = i;
        for j in 2..=i {
            while now % j == 0 {
                *map.entry(j).or_insert(0) += 1;
                now /= j;
            }
        }
    }

    let mut res = 1;
    for (_, v) in map {
        res *= v + 1;
        res %= MOD;
    }

    println!("{}", res);
}
