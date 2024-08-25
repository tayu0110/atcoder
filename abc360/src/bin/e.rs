use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: u32, k: usize}

    let mut c = Modint::zero();
    for i in 1..=n {
        c += Modint::new(2) * Modint::raw(i);
    }

    let n = Modint::raw(n);
    let invn2 = n.pow(2).inv();
    c *= invn2;

    let mut now = Modint::one();
    for _ in 0..k {
        now = c + (n * n - n * Modint::raw(2)) * now * invn2;
    }

    println!("{now}")
}
