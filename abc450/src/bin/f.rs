use std::cmp::Reverse;

use ds::{LazySegmentTree, MapMonoid};
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

struct T;
impl MapMonoid for T {
    type M = (Modint, Modint);
    type Act = Modint;
    fn e() -> Self::M {
        (Modint::zero(), Modint::one())
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0 + r.0, l.1 + r.1)
    }
    fn id() -> Self::Act {
        Modint::zero()
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        *l + *r
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        (m.0 + *act * m.1, m.1)
    }
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (x, y) in e {
        t[x - 1].push(y - 1);
    }

    t.iter_mut()
        .for_each(|t| t.sort_unstable_by_key(|&t| Reverse(t)));

    let mut st = LazySegmentTree::<T>::new(n);
    st.set(0, (Modint::one(), Modint::one()));
    for i in 0..n {
        for &to in &t[i] {
            let now = st.get(i).0;
            st.apply(i..to + 1, now);
        }
    }

    eprintln!("{:?}", (0..n).map(|i| st.get(i).0).collect::<Vec<_>>());
    let ret = st.get(n - 1).0;
    println!("{ret}")
}
