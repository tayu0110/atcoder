use proconio::*;
use segtree::RangeAffineRangeSum;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, m: usize, a: [u64; n], p: [(u32, u32, u64); m]}

    let mut st = RangeAffineRangeSum::new(a.into_iter().map(Modint::new).collect());
    for (l, r, x) in p {
        let p = Modint::one() / (Modint::raw(r) - Modint::raw(l) + Modint::one());
        let x = Modint::new(x);

        st.apply_range(l as usize - 1, r as usize, ((Modint::one() - p), p * x));
    }

    for i in 0..n {
        println!("{}", st.get(i).0.val());
    }
}
