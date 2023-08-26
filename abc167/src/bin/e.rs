use proconio::*;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: u32, m: u32, k: u32}

    let com = combination(2 * n + 10);
    let mut res = Modint::zero();

    for i in n - k..=n {
        let p = Modint::raw(m - 1).pow(i - 1) * Modint::raw(m);
        let lack = n - i;
        res += p * com(n - 1, lack);
    }

    println!("{}", res)
}
