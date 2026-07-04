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
    input! {n: usize, m: usize, a: [usize; n], p: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (b, l, r) in p {
        t[l - 1].push((b, r));
    }

    let mut st = LazySegtree::<T>::new(n + 1);
    st.set(0, 0);
    for i in 0..n {
        let now = st.get(i);
        st.apply(i + 1, now + a[i]);
        for &(b, r) in &t[i] {
            st.apply_range(i + 1, r + 1, now + b);
        }
    }

    println!("{}", st.get(n));
}
