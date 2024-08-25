use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); m]}

    let mut map = FxHashMap::default();
    for (a, b, c) in e {
        map.entry(a).or_insert(vec![]).push((b, c));
        map.entry(b).or_insert(vec![]).push((a, c));
    }

    let mut keys = map.keys().copied().collect::<Vec<_>>();
    keys.push(n);
    keys.push(1);
    keys.dedup();
    keys.sort_unstable();
    for v in keys.windows(2) {
        let a = v[0];
        let b = v[1];
        map.entry(a).or_insert(vec![]).push((b, a.abs_diff(b)));
        map.entry(b).or_insert(vec![]).push((a, a.abs_diff(b)));
    }

    let mut memo = FxHashMap::default();
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 1)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if memo.contains_key(&now) {
            continue;
        }
        memo.insert(now, nd);
        for &(to, c) in map.get(&now).unwrap() {
            if !memo.contains_key(&to) {
                nt.push(Reverse((nd + c, to)));
            }
        }
    }

    println!("{}", memo.get(&n).unwrap());
}
