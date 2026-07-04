use fenwick_tree::{AbelianGroup, FenwickTree};
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

struct T;
impl AbelianGroup for T {
    type G = Modint;

    fn e() -> Self::G {
        Modint::zero()
    }
    fn op(l: &Self::G, r: &Self::G) -> Self::G {
        *l + *r
    }
    fn inv(g: &Self::G) -> Self::G {
        Modint::zero() - *g
    }
}

fn main() {
    input! {n: usize, p: [usize; n]}

    if n < 3 {
        println!("0");
        return;
    }

    let mut dp = vec![
        vec![FenwickTree::<T>::new(n + 1), FenwickTree::<T>::new(n + 1)],
        vec![FenwickTree::<T>::new(n + 1), FenwickTree::<T>::new(n + 1)],
    ];

    let mut count = FenwickTree::<T>::new(n + 1);

    for (i, p) in p.into_iter().enumerate() {
        dp[0][0].add(p, count.fold(p + 1..));
        dp[1][0].add(p, count.fold(..p));
        count.add(p, Modint::one());
        for d in 0..2 {
            let s0 = dp[0][d].fold(p + 1..);
            dp[0][d].add(p, s0);

            let s1 = dp[1][d].fold(..p);
            dp[1][d].add(p, s1);

            if i > 1 {
                if d + 1 < 2 {
                    let s2 = dp[1][d].fold(p + 1..);
                    dp[0][d + 1].add(p, s2);
                }

                if d > 0 {
                    let s3 = dp[0][d].fold(..p);
                    dp[1][d - 1].add(p, s3);
                }
            }
        }
    }

    let mut ret = Modint::zero();
    for c in 0..2 {
        ret += dp[c][1].fold(..);
    }

    println!("{ret}")
}
