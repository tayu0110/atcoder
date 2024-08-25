use proconio::*;
use segtree::{LazySegtree, MapMonoid};

const M: usize = 998244353;

struct SumProduct;

impl MapMonoid for SumProduct {
    type E = (usize, usize, usize, usize);
    type Act = (usize, usize);

    fn e() -> Self::E {
        (0, 0, 0, 0)
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        ((l.0 + r.0) % M, (l.1 + r.1) % M, (l.2 + r.2) % M, l.3 + r.3)
    }

    fn id() -> Self::Act {
        (0, 0)
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        ((l.0 + r.0) % M, (l.1 + r.1) % M)
    }

    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        let (mut s, sa, sb, len) = elem;
        let (ax, bx) = act;

        s += ax * sb % M;
        s %= M;
        s += bx * sa % M;
        s %= M;
        s += ax * bx % M * len % M;
        s %= M;

        (s, (sa + ax * len % M) % M, (sb + bx * len % M) % M, len)
    }
}

fn main() {
    input! {n: usize, q: usize, a: [usize; n], b: [usize; n]}

    let s = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| (a * b % M, a % M, b % M, 1))
        .collect::<Vec<_>>();
    let mut st = LazySegtree::<SumProduct>::from_vec(&s);

    for _ in 0..q {
        input! {ty: u8}
        if ty == 1 {
            input! {l: usize, r: usize, x: usize}
            st.apply_range(l - 1, r, (x, 0));
        } else if ty == 2 {
            input! {l: usize, r: usize, x: usize}
            st.apply_range(l - 1, r, (0, x));
        } else {
            input! {l: usize, r: usize}
            println!("{}", st.prod(l - 1, r).0);
        }
    }
}
