use std::collections::BTreeSet;

use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize, sx: i64, sy: i64, house: [(i64, i64); n], query: [(char, i64); m]}

    let mut row = FxHashMap::default();
    let mut col = FxHashMap::default();
    for (x, y) in house {
        row.entry(y).or_insert_with(|| BTreeSet::new()).insert(x);
        col.entry(x).or_insert_with(|| BTreeSet::new()).insert(y);
    }

    let mut res = 0;
    let (mut r, mut c) = (sy, sx);
    for (query, dist) in query {
        let (nr, nc) = match query {
            'U' => (r + dist, c),
            'D' => (r - dist, c),
            'L' => (r, c - dist),
            'R' => (r, c + dist),
            _ => unreachable!(),
        };

        match query {
            'U' | 'D' => {
                if let Some(set) = col.get_mut(&c) {
                    let (min, max) = (r.min(nr), r.max(nr));
                    let t = set.range(min..=max).cloned().collect::<Vec<_>>();
                    res += t.len();
                    for t in t {
                        set.remove(&t);
                        if let Some(set) = row.get_mut(&t) {
                            set.remove(&c);
                        }
                    }
                }
            }
            'L' | 'R' => {
                if let Some(set) = row.get_mut(&r) {
                    let (min, max) = (c.min(nc), c.max(nc));
                    let t = set.range(min..=max).cloned().collect::<Vec<_>>();
                    res += t.len();
                    for t in t {
                        set.remove(&t);
                        if let Some(set) = col.get_mut(&t) {
                            set.remove(&r);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        (r, c) = (nr, nc);
    }

    println!("{} {} {}", c, r, res);
}
