use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
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

    let mut st = SegmentTree::<T>::new(n);
    st.set(0, 0);
    for (i, &now) in x.iter().enumerate().skip(1) {
        let (l, r) = (
            x.partition_point(|&x| x + r < now),
            x.partition_point(|&x| x + l <= now),
        );
        st.set(i, st.fold(l..r).saturating_add(1));
    }

    println!("{}", st.get(n - 1))
}
