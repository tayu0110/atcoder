use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {h: usize, w: usize}

        let base = Modint::raw(2).pow(2 + 2 * (h as u64 - 1));
        eprintln!("{}", base);
    }
}
