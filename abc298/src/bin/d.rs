use std::collections::VecDeque;

use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {q: usize}

    let mut res = Modint::one();
    let mut ten = Modint::one();
    let mut nt = VecDeque::new();
    nt.push_back(1);
    let t_inv = Modint::raw(10).inv();
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: u32}

            ten *= Modint::raw(10);
            res = res * Modint::raw(10) + Modint::raw(x);
            nt.push_back(x);
        } else if t == 2 {
            let f = nt.pop_front().unwrap();
            res -= Modint::raw(f) * ten;
            ten *= t_inv;
        } else {
            println!("{}", res)
        }
    }
}
