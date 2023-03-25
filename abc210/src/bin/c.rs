#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, k: usize, c: [usize; n]}

    let mut map = std::collections::HashMap::new();
    for &c in c.iter().take(k) {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut res = map.len();
    for (i, &d) in c.iter().enumerate().skip(k) {
        *map.entry(c[i - k]).or_insert(0) -= 1;
        if *map.get(&c[i - k]).unwrap() == 0 {
            map.remove(&c[i - k]);
        }

        *map.entry(d).or_insert(0) += 1;

        res = std::cmp::max(res, map.len());
    }

    println!("{}", res);
}
