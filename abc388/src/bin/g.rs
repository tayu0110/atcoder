use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    type M = i64;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, a: [usize; n], q: usize, query: [(usize, usize); q]}

    let mut next = vec![0; n];
    for (i, &na) in a.iter().enumerate() {
        let pos = a.partition_point(|&a| a < 2 * na);
        next[i] = pos as i64 - i as i64;
    }

    let mut st = SegmentTree::<T>::new(n);
    for i in 0..n {
        st.set(i, next[i]);
    }

    for (l, r) in query {
        let (l, r) = (l - 1, r);
        let (mut lo, mut hi) = (l, r);
        while hi - lo > 1 {
            let m = (hi + lo) / 2;
            if (m - l) * 2 > r - l {
                hi = m;
                continue;
            }
            if st.fold(l..m) <= r as i64 - m as i64 {
                lo = m;
            } else {
                hi = m;
            }
        }
        println!("{}", lo - l)
    }
}
