use polynomial::Polynomial;
use proconio::*;
use static_modint::{Mod998244353, Modulo};

fn main() {
    input! {n: usize, m: u64, mut a: [usize; n]}
    a.push(1);

    let mut poly = Polynomial::<Mod998244353>::from(vec![1]);
    for a in a {
        let mut p = vec![0; a + 1];
        p[0] = 1;
        p[a] = Mod998244353::N - 1;
        poly *= Polynomial::from(p);
    }

    let ret = Polynomial::from(vec![1]).bostan_mori(&poly, m);
    println!("{ret}")
}
