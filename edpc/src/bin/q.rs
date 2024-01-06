use proconio::input;
use segtree::{Monoid, SegmentTree};

struct UsizeMin;

impl Monoid for UsizeMin {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn main() {
    input! {n: usize, h: [usize; n], a: [usize; n]}

    let mut st = SegmentTree::<UsizeMin>::new(n);
    for (h, a) in h.into_iter().zip(a.into_iter()) {
        let max = st.fold(0..h - 1);
        st.set(h - 1, max + a);
    }

    println!("{}", st.fold(0..n))
}
