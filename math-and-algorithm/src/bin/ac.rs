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

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut st = SegmentTree::<UsizeMax>::new(n);
    for (i, a) in a.into_iter().enumerate() {
        let b = if i > 0 { st.fold(0..i - 1) } else { 0 };
        st.set(i, a + b);
    }

    println!("{}", st.fold(0..n))
}
