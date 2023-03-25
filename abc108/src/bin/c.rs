use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut map = HashMap::new();
    for i in 1..=n {
        *map.entry(i % k).or_insert(0) += 1;
    }

    let mut res = 0usize;
    for (&nk, &v) in &map {
        if (nk + nk) % k != 0 {
            continue;
        }
        if let Some(&nv) = map.get(&((k - nk) % k)) {
            res += v * v * nv;
        }
    }

    println!("{}", res)
}
