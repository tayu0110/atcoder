use proconio::*;
use static_modint::{combination, Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {x: u32, y: u32}

    let com = combination(x + y);
    let mut res = Modint::zero();
    for i in 0..=x {
        if i * 2 > y {
            continue;
        }
        let rem_x = x - i;
        let rem_y = y - 2 * i;

        if rem_x % 2 == 1 {
            continue;
        }
        if rem_x / 2 != rem_y {
            continue;
        }

        res += com(i + rem_y, i);
    }

    println!("{}", res)
}
