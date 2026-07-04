use std::{cmp::Reverse, collections::BinaryHeap};

use ds::{EulerTourTree, MapMonoid};
use itertools::Itertools;
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = ();
    type Act = ();
    fn e() -> Self::M {}
    fn op(_: &Self::M, _: &Self::M) -> Self::M {}
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(_: &Self::M, _: &Self::Act) -> Self::M {}
}

fn main() {
    input! {n: usize, q: usize, mut e: [(usize, usize); n-1], query: [(usize, usize); q]}

    let mut e = e
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| (l, r, i))
        .collect::<Vec<_>>();
    e.sort_unstable();
    let mut e = e.into_iter().peekable();
    let mut lct = EulerTourTree::<T>::new(n);

    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b - 1, i))
        .collect::<Vec<_>>();
    query.sort_unstable();

    let mut res = vec![0; q];
    let mut keep = BinaryHeap::new();
    for (a, b, i) in query {
        while let Some((_, r, i)) = e.next_if(|&(l, _, _)| l <= a) {
            lct.link(i, i + 1).ok();
            keep.push(Reverse((r, i)));
        }

        while keep.peek().filter(|&&Reverse((r, _))| r < a).is_some() {
            let Reverse((_, i)) = keep.pop().unwrap();
            lct.cut(i, i + 1);
        }

        res[i] = lct.tree_size(b);
    }

    println!("{}", res.iter().join("\n"))
}
