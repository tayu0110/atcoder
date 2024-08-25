use std::collections::BTreeSet;

use iolib::*;
use rustc_hash::FxHashMap;
use segtree::{Monoid, SegmentTree};

struct U32Min;

impl Monoid for U32Min {
    type M = u32;

    fn id() -> Self::M {
        u32::MAX
    }

    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn main() {
    scan!(n: usize, q: usize, a: [u32; n], query: [(usize, usize, u32); q]);

    let mut st = SegmentTree::<U32Min>::from_vec(a.clone());
    let mut map = FxHashMap::default();
    for (i, &a) in a.iter().enumerate() {
        map.entry(a).or_insert(BTreeSet::new()).insert(i);
    }

    for (t, x, y) in query {
        if t == 1 {
            let old = st.fold(x - 1..x);
            st.set(x - 1, y);
            assert!(map.get_mut(&old).unwrap().remove(&(x - 1)));
            map.entry(y).or_insert(BTreeSet::new()).insert(x - 1);
        } else {
            let min = st.fold(x - 1..y as usize);
            let mut res = map
                .get(&min)
                .unwrap()
                .range(x - 1..y as usize)
                .map(|v| v + 1)
                .collect::<Vec<_>>();
            res.sort_unstable();
            put!(res.len());
            put!(' ');
            putitln!(res.into_iter(), sep = ' ');
        }
    }
}
