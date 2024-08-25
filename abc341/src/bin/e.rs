use proconio::*;
use segtree::{Monoid, SegmentTree};

struct BoolAnd;

impl Monoid for BoolAnd {
    type M = bool;
    fn id() -> Self::M {
        true
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        *l & *r
    }
}

fn main() {
    input! {n: usize, q: usize, s: marker::Bytes}

    if n == 1 {
        for _ in 0..q {
            input! {ty: usize, _: usize, _: usize}
            if ty == 2 {
                println!("Yes");
            }
        }
        return;
    }

    let mut st = SegmentTree::<BoolAnd>::new(n - 1);
    for (i, v) in s.windows(2).enumerate() {
        if v[0] != v[1] {
            st.set(i, true);
        } else {
            st.set(i, false);
        }
    }

    for _ in 0..q {
        input! {ty: u8, l: usize, r: usize}

        if ty == 1 {
            if 1 < l && l < n {
                let prev = l - 2;
                let g = st.fold(prev..prev + 1);
                st.set(prev, !g);
            }
            if 1 < r && r < n {
                let r = r - 1;
                let g = st.fold(r..r + 1);
                st.set(r, !g);
            }
        } else {
            if st.fold(l - 1..r - 1) {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
