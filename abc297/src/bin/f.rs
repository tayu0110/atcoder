use proconio::input;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {h: usize, w: usize, k: usize}

    let com = combination::<Mod998244353>((h * w + 10) as u32);
    let mut res = Modint::zero();
    for i in 1..=h {
        for j in 1..=w {
            let mut t = com(h * w, k);
            t -= com((i - 1) * w, k);
            t -= com(h * (j - 1), k);
            t -= com((h - i) * w, k);
            t -= com(h * (w - j), k);
            t += com((i - 1) * (j - 1), k);
            t += com((i - 1) * (w - j), k);
            t += com((h - i) * (j - 1), k);
            t += com((h - i) * (w - j), k);
            res += t;
        }
    }

    res /= com(h * w, k);
    println!("{}", res)
}
