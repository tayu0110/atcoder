use math::divisors_enumeration;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, s: marker::Chars}

    let mut res = vec![Modint::zero(); n];
    let mut divisors = divisors_enumeration(n as u64);
    divisors.sort();
    divisors.pop();
    for (i, &d) in divisors.iter().enumerate() {
        let mut must = vec![false; d as usize];
        for v in s.chunks_exact(d as usize) {
            for (i, &c) in v.iter().enumerate() {
                if c == '.' {
                    must[i] = true;
                }
            }
        }

        res[d as usize] = Modint::raw(2).pow(must.into_iter().filter(|&f| !f).count() as u64);
        for j in 0..i {
            if d % divisors[j] == 0 {
                res[d as usize] = res[d as usize] - res[divisors[j] as usize];
            }
        }
    }

    println!("{}", res.into_iter().fold(Modint::zero(), |s, v| s + v))
}
