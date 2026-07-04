use ds::{LinkCutTree, MapMonoid};
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = usize;
    type Act = ();
    fn e() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); m], q: usize, mut query: [(usize, usize); q]}

    for (a, b) in p.iter_mut() {
        *a -= 1;
        *b -= 1;
    }
    for (c, d) in query.iter_mut() {
        *c -= 1;
        *d -= 1;
    }

    let mut cycle = LinkCutTree::<T>::new(n * 2);
    for i in (0..n * 2 - 2).step_by(2) {
        cycle.link_flat(i, i + 2).ok();
    }

    let n = n * 2;
    for (a, b) in p {
        let (a, b) = (a.min(b), a.max(b));
        cycle.try_cut((a + n - 1) % n, (a + 1) % n).ok();
        cycle.try_cut((b + n - 1) % n, (b + 1) % n).ok();

        cycle.link_flat((a + n - 1) % n, (b + 1) % n).ok();
        cycle.link_flat((b + n - 1) % n, (a + 1) % n).ok();
    }

    let mut root = vec![0; n];
    for i in 0..n {
        root[i] = cycle.root(i);
    }

    let mut lct = LinkCutTree::<T>::new(n);
    for i in 0..n - 1 {
        if !cycle.is_connected(i, i + 1) {
            lct.link_flat(root[i], root[i + 1]).ok();
        }
    }
    for i in 0..n {
        lct.set(i, 1);
    }

    for (c, d) in query {
        println!("{}", (lct.fold(root[c], root[d]).unwrap() - 1) / 2);
    }
}
