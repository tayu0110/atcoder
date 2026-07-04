use ds::{Monoid, SegmentTree};
use montgomery_modint::Mod998244353;
use proconio::*;
use static_modint::StaticModint;

type Modint = StaticModint<Mod998244353>;

struct T;
impl Monoid for T {
    type M = Modint;
    fn id() -> Self::M {
        Modint::zero()
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        *l + *r
    }
}

fn main() {
    input! {n: usize, c: usize, mut a: [u32; n]}
    let o = a[c - 1];
    a.sort_unstable();
    let c = a.iter().rposition(|&i| i == o).unwrap();
    eprintln!("c: {c}");

    let mut e = vec![Modint::zero(); n];
    let mut cum = a.clone();
    cum.insert(0, 0);
    for i in 0..n {
        cum[i + 1] += cum[i];
    }

    let mut st = SegmentTree::<T>::new(n);
    st.set(
        c,
        (Modint::raw(cum[c]) / Modint::raw(cum[n] - cum[c]) + Modint::one()) * Modint::raw(a[c])
            / Modint::raw(cum[n]),
    );
    for i in c + 1..n {
        e[i] = st.fold(..i);
        st.set(
            i,
            (e[i] + Modint::raw(cum[i]) / Modint::raw(cum[n] - cum[i]) + Modint::one())
                * Modint::raw(a[i])
                / Modint::raw(cum[n]),
        )
    }

    for c in c..n {
        eprintln!(
            "e: {}, {}",
            e[c],
            e[c] + Modint::raw(cum[n]) / Modint::raw(a[c])
        );
    }
    println!("{}", e[n - 1] + Modint::raw(cum[n]) / Modint::raw(a[n - 1]))
}
