use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, mut a: [u32; n]}
    a.sort();

    let mut res = Modint::zero();
    for (i, a) in a.into_iter().map(|a| Modint::raw(a)).enumerate() {
        res += Modint::raw(2).pow(i as u64) * a;
    }

    println!("{}", res)
}
