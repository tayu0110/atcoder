use ds::{LinkCutTree, MapMonoid};
use proconio::*;
use rustc_hash::FxHashMap;

struct EdgeMax;

impl MapMonoid for EdgeMax {
    // (weight, u, v)
    type M = (usize, usize, usize);
    type Act = ();
    fn e() -> Self::M {
        (0, 0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    input! {n: usize, q: usize, e: [(usize, usize, usize); n - 1], query: [(usize, usize, usize); q]}

    let mut res = 0;
    let mut map = FxHashMap::default();
    let mut lct = LinkCutTree::<EdgeMax>::new(2 * n);
    for (i, (a, b, c)) in e.into_iter().enumerate() {
        lct.link_flat(i + n, a - 1).ok();
        lct.link_flat(i + n, b - 1).ok();
        lct.set(i + n, (c, a - 1, b - 1));
        map.insert((a - 1, b - 1), i + n);
        res += c;
    }

    for (u, v, w) in query {
        let &f = lct.fold(u - 1, v - 1).unwrap();

        if f.0 <= w {
            println!("{res}");
            continue;
        }

        let e = map.remove(&(f.1, f.2)).unwrap();
        lct.evert(e);
        lct.cut(f.1);
        lct.cut(f.2);

        lct.link_flat(e, u - 1).ok();
        lct.link_flat(e, v - 1).ok();
        lct.set(e, (w, u - 1, v - 1));
        res += w;
        res -= f.0;
        map.insert((u - 1, v - 1), e);

        println!("{res}");
    }
}
