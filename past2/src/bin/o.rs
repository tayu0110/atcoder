use ds::{LinkCutTree, MapMonoid};
use itertools::Itertools;
use proconio::*;

struct T;

impl MapMonoid for T {
    type M = usize;
    type Act = ();
    fn e() -> Self::M {
        0
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
    input! {n: usize, m: usize, e: [(usize, usize, usize); m]}

    let mut e = e
        .into_iter()
        .enumerate()
        .map(|(i, (a, b, c))| (c, a - 1, b - 1, i))
        .collect::<Vec<_>>();
    e.sort_unstable();

    let mut base = 0;
    let mut lct = LinkCutTree::<T>::new(n * 2 - 1);
    let mut index = n;
    let mut used = vec![false; m];
    for &(c, a, b, i) in &e {
        if !lct.is_connected(a, b) {
            lct.link_flat(a, index).ok();
            lct.link_flat(index, b).ok();
            lct.set(index, c);
            index += 1;
            used[i] = true;
            base += c;
        }
    }

    let mut res = vec![0; m];
    for (c, a, b, i) in e {
        if used[i] {
            res[i] = base;
            continue;
        }

        let max = lct.fold(a, b).unwrap();
        res[i] = base + c - max;
    }

    println!("{}", res.iter().join("\n"))
}
