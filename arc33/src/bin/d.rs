use math::garner;
use modint::{
    Mod1000000007, Mod1811939329, Mod880803841, Mod897581057, Mod998244353, Modulo,
    MontgomeryModint,
};
use polynomial::Polynomial;
use proconio::*;

fn solve<L: Modulo, M: Modulo, N: Modulo>(
    n: usize,
    a: Vec<u32>,
    t: u32,
) -> MontgomeryModint<Mod1000000007> {
    let p = Polynomial::<L>::interpolation(
        (0..n + 1)
            .map(|v| MontgomeryModint::new(v as u32))
            .collect(),
        a.iter()
            .cloned()
            .map(|v| MontgomeryModint::new(v))
            .collect(),
    );
    let p: Vec<u32> = p.into();

    let q = Polynomial::<M>::interpolation(
        (0..n + 1)
            .map(|v| MontgomeryModint::new(v as u32))
            .collect(),
        a.iter()
            .cloned()
            .map(|v| MontgomeryModint::new(v))
            .collect(),
    );
    let q: Vec<u32> = q.into();

    let r = Polynomial::<N>::interpolation(
        (0..n + 1)
            .map(|v| MontgomeryModint::new(v as u32))
            .collect(),
        a.into_iter().map(|v| MontgomeryModint::new(v)).collect(),
    );
    let r: Vec<u32> = r.into();

    let mut res = MontgomeryModint::zero();
    let mut x = MontgomeryModint::<Mod1000000007>::one();
    let ps: [i64; 3] = [L::N as i64, M::N as i64, N::N as i64];
    for ((p, q), r) in p.into_iter().zip(q).zip(r) {
        let (coef, _) = garner(
            &[p as i64, q as i64, r as i64],
            &ps,
            Mod1000000007::N as i64,
        );

        res += x * MontgomeryModint::from(coef);
        x *= MontgomeryModint::new(t);
    }

    res
}

fn main() {
    input! {n: usize, a: [u32; n+1], t: u32}

    println!(
        "{}",
        if a[0] == Mod998244353::N {
            solve::<Mod880803841, Mod897581057, Mod1811939329>(n, a, t)
        } else if a[0] == Mod897581057::N {
            solve::<Mod880803841, Mod998244353, Mod1811939329>(n, a, t)
        } else if a[0] == Mod880803841::N {
            solve::<Mod897581057, Mod998244353, Mod1811939329>(n, a, t)
        } else {
            solve::<Mod880803841, Mod897581057, Mod998244353>(n, a, t)
        }
    )
}
