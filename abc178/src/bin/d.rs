use proconio::*;
use static_modint::{combination, Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {s: u32}

    let com = combination(s + 10);

    let mut res = Modint::zero();
    for len in 1..=s {
        if len * 3 > s {
            break;
        }

        let rem = s - len * 3;
        res += com((rem + len) as u32 - 1, rem as u32);
    }

    println!("{}", res)
}
