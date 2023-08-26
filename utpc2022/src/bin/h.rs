use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: marker::Chars}
    let len = n.len();

    let mut min = vec![10; len + 1];
    let mut pos = vec![len; len + 1];
    for i in (0..len).rev() {
        let c = n[i] as u32 - b'0' as u32;
        if c <= min[i + 1] {
            min[i] = c;
            pos[i] = i;
        } else {
            min[i] = min[i + 1];
            pos[i] = pos[i + 1];
        }
    }

    let mut res = Modint::zero();
    let mut rem = Modint::one();
    let mut now = 0;
    while now < len {
        if min[now] == 9 {
            break;
        }

        res += rem * Modint::raw(9 - min[now]) / Modint::raw(10);
        rem *= Modint::one() / Modint::raw(10);
        now = pos[now] + 1;
    }

    println!("{}", res)
}
