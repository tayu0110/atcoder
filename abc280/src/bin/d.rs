#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {k: u128}

    let mut now = k;
    let mut map = std::collections::BTreeMap::new();
    let mut i = 2;
    while i * i <= k && now != 1 {
        while now % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            now /= i;
        }
        i += 1;
    }

    if now != 1 {
        *map.entry(now).or_insert(0) += 1;
    }

    let mut max = 0;
    for (t, v) in map {
        let (mut l, mut r) = (0, k);
        while r - l > 1 {
            let m = (r + l) / 2;
            let mut now = t;
            let mut sum = 0;
            for _ in 0..v {
                sum += m / now;
                now *= t;
            }

            if sum >= v {
                r = m;
            } else {
                l = m;
            }
        }

        max = std::cmp::max(max, r);
    }

    println!("{}", max);
}
