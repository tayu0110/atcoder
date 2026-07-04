use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    type M = i64;
    fn id() -> Self::M {
        i64::MIN
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

struct U;
impl Monoid for U {
    type M = i64;
    fn id() -> Self::M {
        i64::MAX
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn main() {
    input! {n: usize, k: usize, l: usize, a: [usize; n]}

    let mut st = SegmentTree::<T>::new(n + 1);
    let mut prev = vec![-1; n + 1];
    let mut pos = vec![0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        prev[a] = st.fold(a..).max(prev[i]);
        pos[a] = i as i64;
        st.set(a, i as i64);
    }

    let mut st = SegmentTree::<U>::new(n + 1);
    let mut next = vec![n as i64; n + 1];
    for (i, &a) in a.iter().enumerate() {
        next[a] = st.fold(a..).min(next[i]);
        st.set(a, i as i64);
    }

    let mut cum = vec![0; n + 1];
    for i in 1..=n {
        cum[i] = (next[i] - pos[i]) * (pos[i] - prev[i]);
    }
}
