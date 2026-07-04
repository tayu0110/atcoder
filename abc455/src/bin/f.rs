use ds::{LazySegmentTree, MapMonoid};
use math::MathInt;
use proconio::*;

const M: usize = 998244353;

struct T;
impl MapMonoid for T {
    type M = (usize, usize);
    type Act = usize;
    fn e() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        ((l.0 + r.0) % M, l.1 + r.1)
    }
    fn id() -> Self::Act {
        0
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        (l + r) % M
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        ((m.0 + m.1 * act % M) % M, m.1)
    }
}

struct U;
impl MapMonoid for U {
    type M = (usize, usize, usize);
    type Act = usize;
    fn e() -> Self::M {
        (0, 0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        ((l.0 + r.0) % M, (l.1 + r.1) % M, l.2 + r.2)
    }
    fn id() -> Self::Act {
        0
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        (l + r) % M
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        (
            (m.0 + 2 * m.1 * act % M + act * act % M * m.2 % M) % M,
            (m.1 + m.2 * act % M) % M,
            m.2,
        )
    }
}

fn main() {
    input! {n: usize, q: usize}

    let mut sum = LazySegmentTree::<T>::new(n);
    let mut dsum = LazySegmentTree::<U>::new(n);
    for i in 0..n {
        sum.set(i, (0, 1));
        dsum.set(i, (0, 0, 1));
    }
    for _ in 0..q {
        input! {l: usize, r: usize, a: usize}
        sum.apply(l - 1..r, a);
        dsum.apply(l - 1..r, a);

        let s = sum.fold(l - 1..r).0;
        let ds = dsum.fold(l - 1..r).0;
        // eprintln!("s: {s}, ds: {ds}");
        println!(
            "{}",
            (((s * s % M + M) - ds) * 2usize.inverse_mod(M).unwrap()) % M
        )
    }
}
