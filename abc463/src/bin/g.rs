use cpio::{put, scan};
use static_modint::{combination, Mod998244353, Modulo, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    scan! {t: usize}

    let com = combination::<Mod998244353>(200010);
    for _ in 0..t {
        scan! {n: usize, x: i64}

        let k = Modint::one() / Modint::new(2).pow(n as u64);
        let mut ret = Modint::zero();
        let mut xi = n as i64;
        for i in 0..=n {
            let deg = com(n as u32, i as u32);
            ret += Modint::new((xi - x).abs() as u64 % Mod998244353::N as u64) * deg;
            xi -= 2;
        }

        ret *= k;
        put!(ret.val());
    }
}
