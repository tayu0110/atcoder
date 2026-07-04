use std::collections::BTreeSet;

use ds::{LazySegmentTree, MapMonoid};
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = (usize, usize);
    type Act = usize;
    fn e() -> Self::M {
        (0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0 + r.0, l.1 + r.1)
    }
    fn id() -> Self::Act {
        0
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        l + r
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        (m.0 + m.1 * act, m.1)
    }
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut perm = a
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i + 1))
        .collect::<Vec<_>>();
    perm.sort_unstable();

    let mut st = LazySegmentTree::<T>::new(n);
    for i in 0..n {
        st.set(i, (0, 1));
    }

    let mut used = BTreeSet::new();
    while let Some((a, i)) = perm.pop() {
        let &prev = used.range(..i).next_back().unwrap_or(&0);
        let &next = used.range(i..).next().unwrap_or(&(n + 1));

        let front = i - prev;
        let back = next - i;

        if front < back {
            for i in 0..front {
                st.apply(i..i + back, a);
            }
        } else {
            for i in 0..back {
                st.apply(i..i + front, a);
            }
        }

        used.insert(i);
    }

    for i in 0..n {
        println!("{}", st.get(i).0);
    }
}
