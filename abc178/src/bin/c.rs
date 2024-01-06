use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: u64}
    println!(
        "{}",
        Modint::raw(10).pow(n) - Modint::raw(9).pow(n) * Modint::raw(2) + Modint::raw(8).pow(n)
    )
}
