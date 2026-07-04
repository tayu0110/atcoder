use std::collections::BTreeMap;

use math::MathInt;
use proconio::*;

const M: usize = 1000_000_007;

fn main() {
    input! {n: usize, p: usize, mut a: [usize; n]}

    for a in a.iter_mut() {
        *a %= M;
    }

    let mut map = BTreeMap::new();
    let mut res = 0usize;
    for (i, a) in a.into_iter().enumerate() {
        if a == 0 {
            if p == 0 {
                res += i;
            }
        } else {
            let tar = a.inverse_mod(M).unwrap() * p % M;
            res += *map.get(&tar).unwrap_or(&0);
        }
        *map.entry(a).or_insert(0) += 1;
    }
    println!("{res}")
}
