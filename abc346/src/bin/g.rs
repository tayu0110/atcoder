use std::collections::VecDeque;

use proconio::*;
use rustc_hash::FxHashMap;
use segtree::{LazySegtree, MapMonoid};

#[derive(Debug, Clone, Copy)]
struct LZ;

impl MapMonoid for LZ {
    type E = (usize, usize);
    type Act = usize;

    fn e() -> Self::E {
        (0, 0)
    }
    fn id() -> Self::Act {
        0
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        (l.0 + r.0, l.1 + r.1)
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l.wrapping_add(r)
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        let (mut l, r) = elem;
        l = l.wrapping_add(act);
        if l == 0 {
            (l, 0)
        } else {
            if r == 0 {
                (l, 1)
            } else {
                (l, r)
            }
        }
    }
}

fn main() {
    input! {n: usize, a: [usize; n]}

    {
        let mut map = FxHashMap::default();
        for &a in &a {
            *map.entry(a).or_insert(0usize) += 1;
        }

        if map.values().min().unwrap() == &1 {
            println!("{}", n * (n + 1) / 2);
            return;
        }
    }

    let mut deq = FxHashMap::default();
    for (i, &a) in a.iter().enumerate() {
        let entry = deq.entry(a).or_insert(VecDeque::new());
        entry.push_back(i + 1);
    }

    for deq in deq.values_mut() {
        deq.push_back(n + 1);
    }

    let mut st = LazySegtree::<LZ>::new(n + 2);
    for deq in deq.values() {
        let (mid, r) = (deq[0], deq[1]);
        st.apply_range(mid, r, 1);
    }

    let mut res = 0;
    for a in a {
        let (_, f) = st.prod(0, n + 2);
        res += f;
        let dq = deq.get_mut(&a).unwrap();
        let (mid, r) = (dq[0], dq[1]);
        st.apply_range(mid, r, !0);
        dq.pop_front();
        if dq.len() >= 2 {
            let rr = dq[1];
            st.apply_range(r, rr, 1);
        }
    }

    println!("{}", res);
}
