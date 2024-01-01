use math::factorize;
use proconio::*;
use static_modint::{Mod998244353, Modulo, StaticModint};

type Modint = StaticModint<Mod998244353>;
const M: u128 = Mod998244353::N as u128;

fn main() {
    input! {a: u64, b: u128}

    if b == 0 {
        println!("0");
        return;
    }

    let mut f = factorize(a);
    f.sort();

    let mut t = vec![];
    for &f in &f {
        match t.last_mut() {
            Some((p, cnt)) if *p == f => *cnt += 1,
            _ => t.push((f, 1)),
        }
    }
    t.iter_mut().for_each(|v| v.1 *= b);
    t.sort_by_key(|v| v.1);

    let (_, min) = t.pop().unwrap();
    let u = min / b;
    let mut bu = 1;
    let mut base = Modint::one();
    for (_, w) in t {
        let w = w + 1;
        base *= Modint::raw((w % M) as u32);
        bu *= w;
        bu %= u;
    }
    let (mut p, mut q) = (min, min + 1);
    if p % 2 == 0 {
        p /= 2;
    } else {
        q /= 2;
    }
    let mut t = base * Modint::raw((p % M) as u32) * Modint::raw((q % M) as u32);
    let tu = bu * p % u * q % u;
    t -= Modint::raw((tu % M) as u32);

    println!("{}", t / Modint::raw((u % M) as u32));
}
