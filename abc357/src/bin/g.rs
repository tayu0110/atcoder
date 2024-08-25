use std::collections::HashMap;

use math::MathInt;
use proconio::*;

fn main() {
    const M: u64 = 998244353;
    let sq = M.sqrti();
    let mut map = HashMap::new();
    for i in 0..=sq {
        map.entry(10u64.pow_mod(i, M)).or_insert(vec![]).push(i);
    }

    let inv = 10u64.pow_mod(sq, M).inverse_mod(M).unwrap();
    println!("inv: {inv:?}");
    let mut now = 1;
    for i in 0..=sq {
        if let Some(v) = map.get(&now) {
            for &j in v {
                if i * sq + j > 0 {
                    let res = i * sq + j;
                    assert_eq!(10u64.pow_mod(res, M), 1);
                    println!("{res}");
                }
            }
        }

        now *= inv;
        now %= M;
    }
}
