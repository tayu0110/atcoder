use std::collections::BTreeSet;

use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [usize; n], q: usize, query: [(usize, usize, usize); q]}

    let mut map = FxHashMap::default();
    let mut set = BTreeSet::new();
    for (i, &a) in a.iter().enumerate() {
        set.insert((i + 1, i + 2, a));
        *map.entry(a).or_insert(0) += 1usize;
    }

    let mut res = map
        .values()
        .map(|&v| v * v.saturating_sub(1) / 2)
        .sum::<usize>();
    for (l, mut r, x) in query {
        r += 1;
        let t = set
            .range(..(r, 0, 0))
            .rev()
            .cloned()
            .take_while(|&(_, r, _)| l < r)
            .collect::<Vec<_>>();
        for (pl, pr, px) in t {
            set.remove(&(pl, pr, px));
            let mut pa = *map.get(&px).unwrap();
            res -= pa * pa.saturating_sub(1) / 2;
            pa -= pr - pl;
            res += pa * pa.saturating_sub(1) / 2;
            if r < pr {
                set.insert((r, pr, px));
                res -= pa * pa.saturating_sub(1) / 2;
                pa += pr - r;
                res += pa * pa.saturating_sub(1) / 2;
            }
            if pl < l {
                set.insert((pl, l, px));
                res -= pa * pa.saturating_sub(1) / 2;
                pa += l - pl;
                res += pa * pa.saturating_sub(1) / 2;
            }
            map.insert(px, pa);
        }

        set.insert((l, r, x));
        let mut pa = *map.get(&x).unwrap_or(&0);
        res -= pa * pa.saturating_sub(1) / 2;
        pa += r - l;
        res += pa * pa.saturating_sub(1) / 2;
        map.insert(x, pa);

        println!("{res}")
    }
}
