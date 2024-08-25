use std::cmp::Reverse;

use proconio::*;
use segtree::{Monoid, SegmentTree};

struct SecondMax;

impl Monoid for SecondMax {
    type M = [(i32, i32); 2];

    fn id() -> Self::M {
        [(-1, 0), (-1, 0)]
    }

    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        let mut v = [l[0], l[1], r[0], r[1]];
        v.sort_unstable_by_key(|&v| Reverse(v));
        let mut t = vec![];
        for (a, b) in v {
            match t.last_mut() {
                Some((pc, cnt)) if *pc == a => *cnt += b,
                _ => t.push((a, b)),
            }
        }

        if t.len() >= 2 {
            [t[0], t[1]]
        } else if t.len() == 1 {
            [t[0], (-1, 0)]
        } else {
            Self::id()
        }
    }
}

fn main() {
    input! {n: usize, q: usize, a: [i32; n]}

    let mut st = SegmentTree::<SecondMax>::new(n);
    for (i, &a) in a.iter().enumerate() {
        st.set(i, [(a, 1), (-1, 0)]);
    }

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {p: usize, x: i32}

            st.set(p - 1, [(x, 1), (-1, 0)]);
        } else {
            input! {l: usize, r: usize}

            let [_, res] = st.fold(l - 1..r);

            println!("{}", res.1);
        }
    }
}
