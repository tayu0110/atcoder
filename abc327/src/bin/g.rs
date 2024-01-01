use proconio::*;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: u32, m: u64}

    if n == 1 {
        println!("0");
        return;
    }

    let com = combination(50);
    let mut res = Modint::zero();
    for i in 0..=n {
        res += com(n, i) * (Modint::raw(i) * Modint::raw(n - i)).pow(m);
    }

    println!("{res}")
}
