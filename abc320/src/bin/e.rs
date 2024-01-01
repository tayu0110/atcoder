use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, (usize, usize)); m]}

    let mut map = BTreeMap::from_iter(e);
    for _ in 0..n {
        let mut now = 0;
        let mut res = 0;
        while let Some((&t, &(w, s))) = map.range(now..).next() {
            res += w;
            now = t + s;
            map.remove(&t);
        }

        println!("{res}")
    }
}
