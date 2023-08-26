use montgomery_modint::{Mod998244353, MontgomeryModint};
use polynomial::Polynomial;
use proconio::*;

type Modint = MontgomeryModint<Mod998244353>;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); k]}

    let mut poly = vec![Modint::zero(); n + 1];
    for (l, r) in p {
        for i in l..=r {
            poly[i] = Modint::one();
        }
    }

    let poly: Polynomial<Mod998244353> = poly.into();
    let res = (Polynomial::from(vec![Modint::one()]) - poly).inv(n);

    let res: Vec<u32> = res.into();
    println!("{}", res[n - 1])
}
