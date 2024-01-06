use proconio::*;
use segtree::{LazySegtree, MapMonoid};
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

pub struct RangeAddRangeSumQuery;

impl RangeAddRangeSumQuery {
    pub fn new(size: usize) -> LazySegtree<Self> {
        LazySegtree::from_vec(&vec![(Modint::zero(), Modint::one()); size])
    }
}

impl MapMonoid for RangeAddRangeSumQuery {
    type E = (Modint, Modint);
    type Act = Modint;

    fn e() -> Self::E {
        (Modint::zero(), Modint::zero())
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        (l.0 + r.0, l.1 + r.1)
    }
    fn id() -> Self::Act {
        Modint::zero()
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l + r
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        (elem.0 + act * elem.1, elem.1)
    }
}

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut st = RangeAddRangeSumQuery::new(n + 1);
    st.set(0, (Modint::one(), Modint::one()));

    let inv = Modint::raw(n as u32).inv();
    for i in 0..n {
        let now = st.get(i).0;

        let p = now * inv * Modint::one();

        st.apply_range(i + 1, n + 1, p);
    }

    println!(
        "{}",
        (1..=n).fold(Modint::zero(), |s, v| s + st.get(v).0
            * Modint::raw(a[v - 1]))
    )
}
