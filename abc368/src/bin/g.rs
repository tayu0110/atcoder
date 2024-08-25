use std::collections::BTreeSet;

use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UsizeAdd;

impl Monoid for UsizeAdd {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
}

fn main() {
    input! {n: usize, mut a: [usize; n], mut b: [usize; n], q: usize}

    let mut st = SegmentTree::<UsizeAdd>::from_vec(a.clone());
    let mut pos = BTreeSet::new();
    for (i, &b) in b.iter().enumerate() {
        if b > 1 {
            pos.insert(i);
        }
    }

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {i: usize, x: usize}
            st.set(i - 1, x);
            a[i - 1] = x;
        } else if ty == 2 {
            input! {i: usize, x: usize}
            if b[i - 1] > 1 {
                pos.remove(&(i - 1));
            }
            b[i - 1] = x;
            if x > 1 {
                pos.insert(i - 1);
            }
        } else {
            input! {l: usize, r: usize}
            let mut v = 0;
            let mut prev = l - 1;
            for &m in pos.range(l - 1..r) {
                v += st.fold(prev..m);
                v = (v + a[m]).max(v * b[m]);
                prev = m + 1;
            }
            v += st.fold(prev..r);
            println!("{v}");
        }
    }
}
