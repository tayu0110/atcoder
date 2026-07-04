use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, r: usize, c: usize, n: usize, mut p: [(usize, usize); n]}

    let mut map = BTreeMap::new();
    for &(r, c) in &p {
        map.insert(r, 0);
        map.insert(c, 0);
    }
    let mut cnt = 0;
    for v in map.values_mut() {
        *v = cnt;
        cnt += 1;
    }

    p.sort_unstable();
    for (a, b) in p {}
}
