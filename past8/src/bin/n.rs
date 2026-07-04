use std::collections::BTreeMap;

use fenwick_tree::{AbelianGroup, FenwickTree};
use proconio::*;

const M: usize = 1000_000_007;

struct T;
impl AbelianGroup for T {
    type G = usize;
    fn e() -> Self::G {
        0
    }
    fn op(l: &Self::G, r: &Self::G) -> Self::G {
        l.wrapping_add(*r)
    }
    fn inv(g: &Self::G) -> Self::G {
        g.wrapping_neg()
    }
}

fn main() {
    input! {n: usize, mut a: [usize; n]}

    let map = {
        let mut map = BTreeMap::new();
        for &a in &a {
            map.insert(a, 0);
        }
        map.insert(0, 0);
        map.insert(usize::MAX, 0);
        let mut cnt = 0;
        for (_, v) in map.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        map
    };

    let m = map.len();
    for a in a.iter_mut() {
        let na = *map.get(a).unwrap();
        *a = na;
    }

    let mut dp = vec![FenwickTree::<T>::new(m), FenwickTree::<T>::new(m)];
    dp[0].add(0, 1);
    dp[0].add(m - 1, 1);
    dp[1].add(0, 1);
    dp[1].add(m - 1, 1);
    for a in a {
        let e = dp[0].fold(..a) % M;
        let o = dp[1].fold(a + 1..) % M;
        dp[0].add(a, o);
        dp[1].add(a, e);
    }

    println!(
        "{}",
        (dp[0].fold(1..m - 1) + dp[1].fold(1..m - 1) + M - 2 * n) % M
    )
}
