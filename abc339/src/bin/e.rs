use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UsizeMax;

impl Monoid for UsizeMax {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

const MAX: usize = 500010;

fn main() {
    input! {n: usize, d: usize, a: [usize; n]}

    let mut st = SegmentTree::<UsizeMax>::new(MAX);
    for a in a {
        let lo = a.saturating_sub(d);
        let hi = (a + d).min(MAX - 1);
        let new = st.fold(lo..=hi) + 1;
        st.update_by(a, |old| new.max(*old));
    }

    println!("{}", st.fold(..));
}
