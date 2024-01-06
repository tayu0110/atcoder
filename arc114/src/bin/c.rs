use modint::{Mod998244353, MontgomeryModint};
use proconio::input;

type Modint = MontgomeryModint<Mod998244353>;

fn main() {
    input! {n: usize, m: u32}

    let mut v = vec![Modint::zero(); n];
    for i in 0..n {
        for j in i + 1..n {
            v[j - i] += Modint::one();
        }
    }

    let mut res = Modint::zero();
    for (i, v) in v.into_iter().enumerate() {
        let mut sum = Modint::zero();

        if v == Modint::zero() {
            continue;
        }

        for j in 1..=m {
            sum += Modint::new(m - j).pow(i as u64 - 1);
        }

        res += v * sum * Modint::raw(m).pow(n as u64 - i as u64 - 1);
    }

    println!("{}", Modint::raw(m).pow(n as u64) * (n as u32).into() - res)
}
