use proconio::*;
use segtree::{Monoid, SegmentTree};

struct TupleMax;

impl Monoid for TupleMax {
    type M = (usize, usize);
    fn id() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut st = SegmentTree::<TupleMax>::new(n);
    for i in 0..n {
        st.set(i, (0, usize::MAX - i));
    }

    for a in a {
        st.update_by(a - 1, |&l| (l.0 + 1, l.1));
        let (_, res) = st.fold(0..n);
        println!("{}", usize::MAX - res + 1);
    }
}
