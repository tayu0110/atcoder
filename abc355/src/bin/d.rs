use std::collections::BTreeMap;

use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}

    let mut map = BTreeMap::new();
    for &(l, r) in &p {
        map.insert(l, 0);
        map.insert(r, 0);
    }

    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    p.sort_unstable_by_key(|v| v.1);
    let mut ft = FenwickTree::<Addition<usize>>::new(cnt);
    let mut res = 0;
    for (l, r) in p {
        let l = *map.get(&l).unwrap();
        let r = *map.get(&r).unwrap();
        res += ft.fold(l..=r);
        ft.add(r, 1);
    }

    println!("{res}")
}
