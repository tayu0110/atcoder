use proconio::*;
use segtree::{DynamicSegmentTree, Monoid};

struct I32Add;

impl Monoid for I32Add {
    type M = i32;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
}

fn main() {
    input! {n: usize, p: [(isize, isize); n], q: usize, t: [isize; q]}

    let mut st = DynamicSegmentTree::<I32Add>::new(0..1000010000);
    for (a, b) in p {
        let va = st.fold(a..a + 1);
        st.set(a, va + 1);
        let vb = st.fold(b + 1..b + 2);
        st.set(b + 1, vb - 1);
    }

    for t in t {
        println!("{}", st.fold(0..t + 1));
    }
}
