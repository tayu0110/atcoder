#![allow(unused_imports)]
use math::{mod_log_with_lower_bound_constraint, mod_pow};
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    // for _ in 0..10000 {
    input! {x: i64, p: i64, a: i64, b: i64}

    if b - a < 10000000 {
        let mut min = std::i64::MAX;
        let mut now = mod_pow(x, a, p);
        for _ in a..=b {
            min = std::cmp::min(min, now);
            now = now * x % p;
        }

        println!("{}", min);
        // continue;
        return;
    }

    let mut found = false;
    for i in 0..p {
        if let Some(res) = mod_log_with_lower_bound_constraint(x, i, p, a % (p - 1)) {
            let mut res = res as i128 + (a / (p - 1)) as i128 * (p - 1) as i128;
            if res < a as i128 {
                res += p as i128 - 1;
            }
            if res <= b as i128 {
                println!("{}", i);
                found = true;
                break;
            }
        }
    }

    if !found {
        println!("-1");
    }
    // }
}
