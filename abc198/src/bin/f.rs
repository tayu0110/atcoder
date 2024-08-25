use montgomery_modint::Mod998244353;
use polynomial::Polynomial;
use proconio::*;

type P = Polynomial<Mod998244353>;

fn main() {
    input! {s: u64}

    let p = P::from(vec![1u32, 0, 0, 0, 1, 1, 3]);
    let q = P::from(vec![1, 0, 0, 0, -1])
        * P::from(vec![1, 0, 0, -1]).pow(2, 7)
        * P::from(vec![1, 0, -1]).pow(2, 5)
        * P::from(vec![1, -1]);
    println!("{}", p.bostan_mori(&q, s - 6));
}
