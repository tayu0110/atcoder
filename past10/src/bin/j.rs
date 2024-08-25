use proconio::*;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, k: u32, mut a: [u32; n]}

    a.sort_unstable();
    let com = combination::<Mod998244353>(n as u32 + 10);
    let mut res = Modint::zero();
    for i in 0..n {
        res -= com((n - 1 - i) as u32, k - 1) * Modint::raw(a[i]);
    }
    a.reverse();
    for i in 0..n {
        res += com((n - 1 - i) as u32, k - 1) * Modint::raw(a[i]);
    }

    println!("{}", res / com(n as u32, k));
}
