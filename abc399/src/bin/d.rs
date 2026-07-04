use itertools::Itertools;
use proconio::*;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {n: usize, a: [usize; n * 2]}

        let mut map = FxHashMap::default();
        let mut bad = FxHashSet::default();
        for (i, v) in a.windows(2).enumerate() {
            let a = v[0];
            let b = v[1];
            if a == b {
                bad.insert(a);
                continue;
            }
            map.entry((a.min(b), a.max(b)))
                .or_insert_with(|| vec![])
                .push(i);
        }

        let mut cnt = 0;
        for ((k1, k2), v) in map {
            if v.len() < 2 {
                continue;
            }

            if bad.contains(&k1) || bad.contains(&k2) {
                continue;
            }

            let mut ok = false;
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    ok |= v[i].abs_diff(v[j]) >= 2;
                }
            }
            if ok {
                cnt += 1;
            }
        }
        res.push(cnt);
    }

    println!("{}", res.iter().join("\n"))
}
