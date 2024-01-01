use proconio::*;
use segtree::SegmentTree;
use std::collections::BTreeMap;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    let mut map = BTreeMap::new();
    for &a in &a {
        map.insert(a, 0);
    }

    {
        let mut cnt = 0;
        for (_, v) in map.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
    }

    for a in a.iter_mut() {
        *a = *map.get(&a).unwrap();
    }

    let mut st = SegmentTree::new(n, 0, |l, r| std::cmp::max(*l, *r));
    for (i, a) in a.into_iter().enumerate() {
        let max = st.foldl(a, n);
        if max != 0 {
            println!("{max}");
        } else {
            println!("-1")
        }
        st.set(a, i + 1);
    }
}
