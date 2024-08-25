use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut cum = vec![0; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        cum[i + 1] = cum[i] + a;
    }
    cum.sort_unstable();

    let mut res = Modint::zero();
    for (i, c) in cum.into_iter().map(Modint::new_signed).enumerate() {
        res += c * Modint::new(i as u64);
        res -= c * Modint::new(n as u64 - i as u64);
    }

    println!("{res}");
}
