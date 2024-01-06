use proconio::*;
use segtree::SegmentTree;

struct UsizeMin;
impl segtree::Monoid for UsizeMin {
    type M = usize;
    fn id() -> Self::M {
        usize::MAX
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn main() {
    input! {n: usize, l: usize, r: usize, x: [usize; n]}

    let mut st = SegmentTree::<UsizeMin>::new(n);
    st.set(0, 0);
    for (_i, &now) in x.iter().enumerate().skip(1) {
        let _nl = x.partition_point(|&x| x <= now.saturating_sub(r));
        let _nr = x.partition_point(|&x| x <= now.saturating_sub(l));
    }
}
