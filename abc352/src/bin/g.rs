use montgomery_modint::{Mod998244353, MontgomeryModint};
use polynomial::Polynomial;
use proconio::*;

type Modint = MontgomeryModint<Mod998244353>;

fn prod(a: &[u32]) -> Polynomial<Mod998244353> {
    if a.len() == 1 {
        return Polynomial::from(vec![1, a[0]]);
    }
    let m = a.len() / 2;
    prod(&a[..m]) * prod(&a[m..])
}

fn main() {
    input! {n: usize, a: [u32; n]}

    let s = a.iter().sum::<u32>();

    let f = prod(&a);
    let mut inv = (0..=n)
        .map(|i| Modint::new(s - i as u32))
        .collect::<Vec<_>>();
    for i in 1..=n {
        inv[i] = inv[i] * inv[i - 1];
    }
    inv[n] = inv[n].inv();
    for i in (0..n).rev() {
        inv[i] = inv[i + 1] * Modint::new(s - 1 - i as u32);
    }
    inv.insert(0, Modint::one());
    let mut res = Modint::zero();
    let mut com = Modint::one();
    for i in 0..=n {
        res += f[i] * com * inv[i];
        com *= Modint::new(i as u32 + 1);
    }

    println!("{res}")
}
