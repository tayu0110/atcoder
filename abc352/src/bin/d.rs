use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UsizeMinMax;

impl Monoid for UsizeMinMax {
    // (min, max)
    type M = (usize, usize);

    fn id() -> Self::M {
        (usize::MAX, 0)
    }

    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0.min(r.0), l.1.max(r.1))
    }
}

fn main() {
    input! {n: usize, k: usize, p: [usize; n]}

    let mut st = SegmentTree::<UsizeMinMax>::new(n);
    for (i, p) in p.into_iter().enumerate() {
        st.set(p - 1, (i, i));
    }

    let mut res = usize::MAX;
    for i in 0..n {
        if i + k > n {
            break;
        }

        let (min, max) = st.fold(i..i + k);
        res = res.min(max - min);
    }

    println!("{}", res)
}
