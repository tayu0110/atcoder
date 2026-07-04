use ds::{LazySegmentTree, MapMonoid};
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = (usize, usize);
    type Act = usize;
    fn e() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0 + r.0, l.1 + r.1)
    }
    fn id() -> Self::Act {
        0
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        l + r
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        (m.0 + act * m.1, m.1)
    }
}

const MAX: usize = 1000100;

fn main() {
    input! {n: usize, p: [(usize, usize); n], q: usize}

    let mut st = LazySegmentTree::<T>::new(MAX);
    for i in 0..MAX {
        st.set(i, (i, 1));
    }

    for (l, r) in p {
        let (ol, or) = (l, r);
        let l = {
            let (mut l, mut r) = (0, MAX);
            while r - l > 1 {
                let m = (r + l) / 2;
                if st.get(m).0 < ol {
                    l = m;
                } else {
                    r = m;
                }
            }
            r
        };
        let r = {
            let (mut l, mut r) = (0, MAX);
            while r - l > 1 {
                let m = (r + l) / 2;
                if st.get(m).0 <= or {
                    l = m;
                } else {
                    r = m;
                }
            }
            r
        };
        st.apply(l..r, 1);
    }

    for _ in 0..q {
        input! {x: usize}
        println!("{}", st.get(x).0);
    }
}
