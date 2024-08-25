use ds::{DynamicSequence, MapMonoid};
use proconio::*;
use segtree::{Monoid, SegmentTree};

struct T;
impl MapMonoid for T {
    type M = usize;
    type Act = ();
    fn e() -> Self::M {
        0
    }
    fn op(l: &Self::M, _: &Self::M) -> Self::M {
        *l
    }
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

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

    let mut seq = DynamicSequence::<T>::new(0);
    for (i, a) in a.into_iter().enumerate() {
        seq.insert(i - a, i);
    }

    let mut st = SegmentTree::<UsizeMax>::new(n);
    while let Some(a) = seq.pop_first() {
        st.set(a, st.fold(..a) + 1);
    }

    println!("{}", st.fold(..))
}
