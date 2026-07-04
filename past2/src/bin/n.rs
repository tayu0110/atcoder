use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use ds::{LazySegmentTree, MapMonoid};
use itertools::Itertools;
use proconio::*;

struct T;
impl MapMonoid for T {
    type Act = i64;
    type M = (i64, i64);
    fn e() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0 + r.0, l.1 + r.1)
    }

    fn id() -> Self::Act {
        0
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        l + r
    }

    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        (m.0 + m.1 * act, m.1)
    }
}

fn main() {
    input! {n: usize, q: usize, mut p: [(i64, i64, i64, i64); n], query: [(i64, i64); q]}
    p.sort_unstable();

    let mut map = BTreeMap::new();
    for &(_, ymin, d, _) in &p {
        map.insert(ymin, 0);
        map.insert(ymin + d, 0);
    }
    for &(_, y) in &query {
        map.insert(y, 0);
    }
    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b, i))
        .collect::<Vec<_>>();
    query.sort_unstable();

    let mut res = vec![0; q];
    let mut next_check = 0;
    let mut keep = BinaryHeap::new();
    let mut st = (0..cnt).map(|_| (0, 1)).collect::<LazySegmentTree<T>>();
    for (a, b, i) in query {
        while next_check < n && p[next_check].0 <= a {
            let (xmin, ymin, d, c) = p[next_check];
            next_check += 1;
            keep.push(Reverse((xmin + d, ymin, d, c)));
            st.apply(*map.get(&ymin).unwrap()..=*map.get(&(ymin + d)).unwrap(), c);
        }
        while keep.peek().filter(|v| v.0 .0 < a).is_some() {
            let Reverse((_, ymin, d, c)) = keep.pop().unwrap();
            st.apply(
                *map.get(&ymin).unwrap()..=*map.get(&(ymin + d)).unwrap(),
                -c,
            );
        }

        res[i] = st.get(*map.get(&b).unwrap()).0;
    }

    println!("{}", res.iter().join("\n"))
}
