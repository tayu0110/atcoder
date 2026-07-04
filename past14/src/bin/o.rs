use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct T;

impl MapMonoid for T {
    type E = (u32, u32);
    type Act = u32;

    fn e() -> Self::E {
        (0, 0)
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        (l.0 + r.0, l.1 + r.1)
    }

    fn id() -> Self::Act {
        u32::MAX
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        if l == u32::MAX {
            r
        } else {
            l
        }
    }

    fn map(act: Self::Act, mut elem: Self::E) -> Self::E {
        if act != u32::MAX {
            elem.0 = act * elem.1;
        }
        elem
    }
}

fn main() {
    input! {n: usize, q: usize, a: [u8; n], query: [(u8, usize, usize); q]}

    let mut st = (0..11)
        .map(|_| LazySegtree::<T>::new(n))
        .collect::<Vec<_>>();
    for i in 0..11 {
        for j in 0..n {
            st[i].set(j, (0, 1));
        }
    }
    for (i, a) in a.into_iter().enumerate() {
        st[a as usize].set(i, (a as u32, 1));
    }
    for (c, mut l, mut r) in query {
        l -= 1;
        if c == 1 {
            let nr = r;
            for i in (1..11).rev() {
                let s = st[i].prod(l, nr).0 / i as u32;
                st[i].apply_range(l, nr, 0);
                st[i].apply_range(r - s as usize, r, i as u32);
                r -= s as usize;
            }
        } else if c == 2 {
            let nl = l;
            for i in (1..11).rev() {
                let s = st[i].prod(nl, r).0 / i as u32;
                st[i].apply_range(nl, r, 0);
                st[i].apply_range(l, l + s as usize, i as u32);
                l += s as usize;
            }
        } else {
            let mut sum = 0;
            for i in 0..11 {
                sum += st[i].prod(l, r).0;
            }
            println!("{sum}")
        }
    }
}
