use montgomery_modint::Mod998244353;
use proconio::*;
use static_modint::{combination, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {a: u32, b: u32, c: u32, d: u32}

    let com = combination::<Mod998244353>(a + b + c + d + 10);
    let mut res = Modint::zero();
    for bpos in b..=a + b + c {
        let len = bpos - 1;
        let bp = com(len, b - 1);
        let last = bpos.max(a + b);
        let rem = a + b + c + d - last;
        res += bp * com(rem, d);
    }

    println!("{res}");
}
