use montgomery_modint::Modulo;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: u64}

    let nn = Modint::from(n);
    let p = Modint::raw(3).pow(n % (Mod998244353::N as u64 - 1));
    println!(
        "{}",
        (p - nn * Modint::raw(2) - Modint::one()) / Modint::raw(4)
    );
}
