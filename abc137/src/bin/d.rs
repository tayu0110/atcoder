#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: i32, mut p: [(i32, usize); n]}
    let mut p = p.into_iter().map(|(a, b)| (b, a)).collect::<Vec<_>>();
    p.sort();

    let mut res = 0;
    let mut set = (0..m).collect::<std::collections::BTreeSet<i32>>();
    while let Some((b, a)) = p.pop() {
        if a > m {
            continue;
        }

        let rem = m - a;
        if let Some(&k) = set.range(0..=rem).next_back() {
            set.remove(&k);
            res += b;
        }
    }

    println!("{}", res);
}
