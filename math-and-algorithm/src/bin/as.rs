use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: u64}

    println!(
        "{}",
        (Modint::raw(4).pow(n + 1) - Modint::one()) / Modint::raw(3)
    )
}
