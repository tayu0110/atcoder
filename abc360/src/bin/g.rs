use std::collections::BTreeMap;

use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UsizeMax;

impl Monoid for UsizeMax {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let (m, map) = {
        let mut map = BTreeMap::new();
        for &a in &a {
            map.insert(a, 0);
            map.insert(a + 1, 0);
        }
        let mut now = 1;
        for (_, v) in map.iter_mut() {
            *v = now;
            now += 1;
        }
        (now, map)
    };

    let mut dp = vec![
        SegmentTree::<UsizeMax>::new(m),
        SegmentTree::<UsizeMax>::new(m),
    ];
    let mut prev = usize::MAX;
    for a in a {
        let &ma = map.get(&a).unwrap();
        for i in 0..2 {
            let max = dp[i].fold(..ma);
            dp[i].update_by(ma, |&old| old.max(max + 1));
        }

        {
            let ma = prev.wrapping_add(1);
            let max = dp[0].fold(..ma);
            dp[1].update_by(ma, |&old| old.max(max + 1));
        }

        prev = ma;
    }

    println!("{}", dp[0].fold(..).max(dp[1].fold(..)));
}
