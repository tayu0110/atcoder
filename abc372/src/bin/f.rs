use proconio::*;
use rustc_hash::FxHashMap;

const M: usize = 998244353;

fn main() {
    input! {n: usize, m: usize, k: usize, mut e: [(usize, usize); m]}
    e.iter_mut().for_each(|e| {
        e.0 -= 1;
        e.1 -= 1;
    });

    let mut memo = FxHashMap::default();
    memo.insert(0, 1);
    for _ in 0..k {
        let mut next = FxHashMap::default();

        for &(from, to) in &e {
            let entry = next.entry((to + n - 1) % n).or_insert(0);
            *entry += *memo.get(&from).unwrap_or(&0);
            *entry %= M;
        }

        for (f, t) in e.iter_mut() {
            *f = (*f + n - 1) % n;
            *t = (*t + n - 1) % n;
        }

        for (k, v) in next {
            let entry = memo.entry(k).or_insert(0);
            *entry += v;
            *entry %= M;
        }
    }

    println!("{}", memo.values().fold(0, |s, v| (s + v) % M))
}
