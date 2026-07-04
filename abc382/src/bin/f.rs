use itertools::Itertools;
use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct T;
impl MapMonoid for T {
    type E = usize;
    type Act = usize;
    fn e() -> Self::E {
        usize::MAX
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        l.min(r)
    }

    fn id() -> Self::Act {
        usize::MAX
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l.min(r)
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        elem.min(act)
    }
}

fn main() {
    input! {h: usize, w: usize, n: usize, bar: [(usize, usize, usize); n]}

    let mut bar = bar
        .into_iter()
        .enumerate()
        .map(|(i, (r, c, l))| (i, r, c, l))
        .collect::<Vec<_>>();
    bar.sort_unstable_by_key(|&(i, r, c, l)| (r, c, l, i));
    let mut st = LazySegtree::<T>::new(w + 1);
    let mut res = vec![0; n];
    while let Some((i, _, c, l)) = bar.pop() {
        let min = st.prod(c, c + l).min(h + 1);
        res[i] = min - 1;
        st.apply_range(c, c + l, min - 1);
    }

    println!("{}", res.iter().join("\n"))
}
