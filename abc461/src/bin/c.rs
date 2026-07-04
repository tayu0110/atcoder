use std::cmp::Reverse;

use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, k: usize, m: usize, mut e: [(usize, usize); n]}

    e.sort_unstable_by_key(|e| Reverse(e.1));
    let mut top = vec![];
    let mut set = FxHashSet::default();
    let mut second = vec![];
    for (c, v) in e {
        if set.insert(c) {
            top.push(v);
        } else {
            second.push(v);
        }
    }
    top.sort_unstable_by_key(|&t| Reverse(t));
    let mut ret = top.drain(..k.min(m)).sum::<usize>();
    second.extend(top);
    second.sort_unstable_by_key(|&t| Reverse(t));
    ret += second.into_iter().take(k.saturating_sub(m)).sum::<usize>();
    println!("{ret}")
}
