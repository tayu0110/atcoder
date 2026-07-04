use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, mut e: [(i32, i32); n]}

    e.sort_unstable_by_key(|e| (e.0.abs_diff(e.1)));

    let mut set = BTreeSet::new();
    let mut ret = 0;
    for (a, b) in e {
        let min = a.min(b);
        let max = a.max(b);

        if let Some(&(s, _)) = set.range((min, min)..).next() {
            if s <= max {
                continue;
            }
        }

        if let Some(&(_, e)) = set.range(..=(max, max)).next_back() {
            if min <= e {
                continue;
            }
        }

        set.insert((min, max));
        ret += 1;
    }

    println!("{ret}")
}
