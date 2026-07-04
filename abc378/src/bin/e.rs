use proconio::*;
use segtree::{Monoid, SegmentTree};

struct T;

impl Monoid for T {
    type M = (usize, usize);
    fn id() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0 + r.0, l.1 + r.1)
    }
}

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut res = 0;
    let mut st = SegmentTree::<T>::new(m + 2);
    let mut total = 0;
    st.set(0, (0, 1));
    for a in a {
        total = (total + a) % m;
        // less
        {
            let (sum, cnt) = st.fold(..total);
            res += total * cnt - sum;
        }
        // more
        {
            let (sum, cnt) = st.fold(total + 1..);
            res += (m + total) * cnt - sum;
        }
        st.update_by(total, |(s, c)| (s + total, c + 1));
    }

    println!("{res}")
}
