use std::collections::VecDeque;

use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    type M = i32;
    fn id() -> Self::M {
        i32::MIN
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, d: usize, r: usize, h: [usize; n]}

    let mut st = SegmentTree::<T>::new(n);
    let mut delayed = VecDeque::new();
    let mut pos = vec![0; n + 1];
    for (i, &h) in h.iter().enumerate() {
        pos[h] = i;
    }

    for i in 1..n + 1 {
        let p = pos[i];
        let min = p.saturating_sub(r);
        let max = (p + r).min(n - 1);

        let m = st.fold(min..=max);
        delayed.push_back(((m + 1).max(0), p));
        if delayed.len() >= d {
            let (m, pos) = delayed.pop_front().unwrap();
            st.set(pos, m);
        }
    }

    for (m, pos) in delayed {
        st.set(pos, m);
    }

    println!("{}", st.fold(..))
}
