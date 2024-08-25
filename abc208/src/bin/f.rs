use montgomery_modint::{Mod1000000007, MontgomeryModint};
use polynomial::interpolation_with_eval;
use proconio::*;

type Modint = MontgomeryModint<Mod1000000007>;

fn main() {
    input! {n: u64, m: usize, k: usize}

    let mut cum = vec![Modint::zero(); k + m + 1];
    for (i, c) in cum.iter_mut().enumerate().skip(1) {
        *c = Modint::new(i as u32).pow(k as u64);
    }
    for _ in 1..m + 1 {
        for j in 0..k + m {
            cum[j + 1] = cum[j + 1] + cum[j];
        }
    }

    println!("{}", interpolation_with_eval(cum, Modint::from(n)));
}
