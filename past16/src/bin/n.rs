use std::collections::BTreeMap;

use cpio::putln;
use math::MathInt;
use proconio::*;
use segtree::{LazySegtree, MapMonoid};

const M: usize = 998244353;

struct T;
impl MapMonoid for T {
    type E = (usize, usize);
    type Act = usize;

    fn e() -> Self::E {
        (0, 0)
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        (l.0 + r.0, l.1 + r.1)
    }

    fn id() -> Self::Act {
        1
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l * r % M
    }

    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        (elem.0 * act % M, elem.1)
    }
}

fn main() {
    input! {q: usize, k: usize, query: [(char, usize); q]}

    let map = {
        let mut map = BTreeMap::new();
        for &(_, x) in &query {
            *map.entry(x).or_insert(0) += 1;
        }
        let mut cnt = 0;
        for (_, v) in map.iter_mut() {
            let old = *v;
            *v = cnt;
            cnt += old;
        }
        map
    };

    let ik = k.inverse_mod(M).unwrap();
    let powk = {
        let mut buf = vec![0; q];
        buf[0] = 1;
        for i in 1..q {
            buf[i] = buf[i - 1] * k % M;
        }
        buf
    };
    let mut st = LazySegtree::<T>::new(q);
    let mut cnt = BTreeMap::new();
    for (op, x) in query {
        let base = *map.get(&x).unwrap();
        let entry = cnt.entry(x).or_insert(0);
        if op == '+' {
            let used = *entry;
            st.apply_range(base + used, q, k);
            let (_, low) = st.prod(0, base + used);
            st.set(base + used, (x * powk[low] % M, 1));
            *entry += 1;
        } else {
            *entry -= 1;
            let used = *entry;
            st.set(base + used, (0, 0));
            st.apply_range(base + used, q, ik);
        }
        putln!(st.prod(0, q).0 % M);
    }
}
