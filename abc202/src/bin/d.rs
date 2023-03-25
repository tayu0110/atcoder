#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn comb(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    let (n, k) = (n as u128, k as u128);
    let mut res = 1;
    let mut set = std::collections::HashSet::new();
    for n in (1..=n).rev().take(k as usize) {
        res *= n;
        for k in 1..=k {
            if !set.contains(&k) && res % k == 0 {
                res /= k;
                set.insert(k);
            }
        }
    }

    for k in 1..=k {
        if !set.contains(&k) {
            res /= k;
        }
    }
    res as usize
}

fn main() {
    input! {a: usize, b: usize, mut k: usize}
    k -= 1;

    let mut res = String::new();
    let mut rem_a = a;
    let mut rem_b = b;
    for _ in 0..a + b {
        if rem_a == 0 {
            res.push('b');
        } else if rem_b == 0 {
            res.push('a');
        } else {
            let c = comb(rem_a + rem_b - 1, rem_a - 1);
            if c > k {
                res.push('a');
                rem_a -= 1;
            } else {
                res.push('b');
                k -= c;
                rem_b -= 1;
            }
        }
    }

    println!("{}", res);
}
