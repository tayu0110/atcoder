use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: u32, q: usize}

    for _ in 0..q {
        input! {a: u32, d: u32, k: u32}

        let mut com = Modint::one();
        let mut den = Modint::raw(n - 1);
        let mut num = Modint::raw(k - 1);
        while num != Modint::zero() {
            com *= den;
            com /= num;
            den -= Modint::one();
            num -= Modint::one();
        }

        eprintln!("com: {com:?}");

        com *= Modint::raw(n) * Modint::raw(a)
            + Modint::raw(d) * Modint::raw(n) * Modint::raw(n - 1) / Modint::raw(2);
        println!("{}", com);
    }
}
