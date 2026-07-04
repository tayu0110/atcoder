use std::collections::BTreeMap;

use proconio::*;
use segtree::{Monoid, SegmentTree};

struct I64Sum;

impl Monoid for I64Sum {
    type M = i64;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
}

fn main() {
    input! {n: usize, x: [isize; n], p: [i64; n], q: usize, query: [(isize, isize); q]}

    let mut map = BTreeMap::new();
    for &x in &x {
        map.insert(x, 0);
    }
    for &(l, r) in &query {
        map.insert(l, 0);
        map.insert(r, 0);
    }
    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    let mut st = SegmentTree::<I64Sum>::new(cnt);
    for (x, p) in x.into_iter().zip(p) {
        let x = *map.get(&x).unwrap();
        st.update_by(x, |old| old + p);
    }

    for (l, r) in query {
        let l = *map.get(&l).unwrap();
        let r = *map.get(&r).unwrap();
        println!("{}", st.fold(l..=r))
    }
}
