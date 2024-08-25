use std::collections::BTreeMap;

use fenwick_tree::FenwickTree;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let map = {
        let mut mp = BTreeMap::new();
        for &a in &a {
            mp.insert(a, 0);
        }

        let mut cnt = 0;
        for (_, v) in mp.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        mp
    };

    let mut res = 0;
    let mut ft = FenwickTree::<usize>::new(n, 0);
    let mut cnt = FenwickTree::<usize>::new(n, 0);
    for a in a {
        let &idx = map.get(&a).unwrap();

        res += a * cnt.get_sum(0, idx) - ft.get_sum(0, idx);
        ft.add(idx, a);
        cnt.add(idx, 1);
    }

    println!("{res}")
}
