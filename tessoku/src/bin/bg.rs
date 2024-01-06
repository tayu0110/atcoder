use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UsizeAdd;

impl Monoid for UsizeAdd {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
}

fn main() {
    input! {n: usize, q: usize}

    let mut st = SegmentTree::<UsizeAdd>::new(n);
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {pos: usize, x: usize}
            st.set(pos - 1, x);
        } else {
            input! {l: usize, r: usize}
            println!("{}", st.fold(l - 1..r - 1))
        }
    }
}
