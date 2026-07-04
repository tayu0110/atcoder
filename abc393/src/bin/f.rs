use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use ds::{Monoid, SegmentTree};
use itertools::Itertools;
use proconio::*;

struct T;
impl Monoid for T {
    type M = (usize, usize);
    fn id() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        if l.0 >= r.0 {
            *l
        } else {
            *r
        }
    }
}

fn main() {
    input! {n: usize, q: usize, a: [usize; n], query: [(usize, usize); q]}

    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, (r, x))| (x, r, i))
        .collect::<Vec<_>>();
    query.sort_unstable_by_key(|&q| Reverse(q));

    let mut nt = a
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, a)| Reverse((a, i)))
        .collect::<BinaryHeap<_>>();

    let mut st = SegmentTree::<T>::new(n);
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut res = vec![0; q];
    while let Some((x, r, i)) = query.pop() {
        let min = x;
        while nt.peek().map_or(false, |nt| nt.0 .0 <= min) {
            let Reverse((na, i)) = nt.pop().unwrap();

            if let Some(&(mut end, mut start)) = set.range(..(i, i)).next_back() {
                if end + 1 == i && a[end] < na {
                    set.remove(&(end, start));
                    st.set(start, (0, 0));
                    end = i;
                } else {
                    start = i;
                    end = i;
                }

                if let Some(&(nend, nstart)) = set.range((i, i)..).next() {
                    if i + 1 == nstart && na < a[nstart] {
                        set.remove(&(nend, nstart));
                        st.set(nstart, (0, 0));
                        end = nend;
                    }
                }
                set.insert((end, start));
                st.set(start, (end + 1 - start, start));
            } else if let Some(&(mut end, mut start)) = set.range((i, i)..).next() {
                if i + 1 == start && na < a[start] {
                    set.remove(&(end, start));
                    st.set(start, (0, 0));
                    start = i;
                } else {
                    start = i;
                    end = i;
                }
                set.insert((end, start));
                st.set(start, (end + 1 - start, start));
            } else {
                set.insert((i, i));
                st.set(i, (1, i));
            }
        }

        let (mut len, start) = st.fold(..r);
        if start + len >= r {
            len = r - start;

            let (l, _) = st.fold(..start);
            len = len.max(l);
        }
        res[i] = len;
    }

    println!("{}", res.iter().join("\n"))
}
