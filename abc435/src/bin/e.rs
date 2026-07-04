use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, query: [(usize, usize); q]}

    let mut list = vec![(1, false), (n, true)];
    for &(l, r) in &query {
        list.push((l, false));
        list.push((r, true));
    }
    list.sort_unstable();
    list.dedup();

    let mut set = BTreeSet::new();
    for v in list.windows(2) {
        match (v[0].1, v[1].1) {
            (false, false) => set.insert((v[1].0 - 1, v[0].0)),
            (false, true) => set.insert((v[1].0, v[0].0)),
            (true, false) => {
                if v[0].0 + 1 > v[1].0 - 1 {
                    continue;
                }
                set.insert((v[1].0 - 1, v[0].0 + 1))
            }
            (true, true) => set.insert((v[1].0, v[0].0 + 1)),
        };
    }

    let mut white = n;
    for (l, r) in query {
        while let Some(&(s, e)) = set.range(..=(r, r)).next_back() {
            if (l..=r).contains(&s) && (l..=r).contains(&e) {
                white -= s + 1 - e;
                set.remove(&(s, e));
            } else {
                break;
            }
        }

        println!("{white}")
    }
}
