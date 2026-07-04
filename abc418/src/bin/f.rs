use std::collections::BTreeSet;

use itertools::Itertools;
use montgomery_modint::Mod998244353;
use proconio::*;
use static_modint::{combination, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, q: usize, query: [(usize, i32); q]}

    let mut com = combination::<Mod998244353>(1000000);
    let mut map = BTreeSet::new();
    let mut res = Modint::zero();
    let mut memo = vec![-1; n + 1];
    for (x, y) in query {
        if y < 0 {
            let prev = map.range(..x).next_back();
            let next = map.range(x..).next();

            map.remove(&x);
        } else {
            let prev = map.range(..x).next_back();
            let next = map.range(x..).next();
            map.insert(x);
            memo[x] = y;
        }
    }
}
