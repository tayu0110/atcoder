use std::cmp::Reverse;

use ds::{DynamicSortedSequence, MapMonoid};
use proconio::*;
use rustc_hash::FxHashMap;

struct T;
impl MapMonoid for T {
    type Act = ();
    type M = Reverse<usize>;
    fn e() -> Self::M {
        Reverse(0)
    }
    fn op(l: &Self::M, _: &Self::M) -> Self::M {
        *l
    }
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    input! {n: usize, q: usize, mut p: [usize; n]}

    let mut map = FxHashMap::default();
    for (i, &p) in p.iter().enumerate() {
        map.insert(p, i);
    }
    let mut seq = p
        .iter()
        .cloned()
        .map(|p| Reverse(p))
        .collect::<DynamicSortedSequence<T>>();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {a: usize, x: usize}
            seq.remove_once(&Reverse(p[a - 1]));
            seq.insert(Reverse(x));
            p[a - 1] = x;
            map.insert(x, a - 1);
        } else if ty == 2 {
            input! {a: usize}
            let index = seq.first_index_of(&Reverse(p[a - 1])).unwrap();
            println!("{}", index + 1);
        } else {
            input! {r: usize}
            let Reverse(p) = seq.get(r - 1).unwrap();
            println!("{}", map.get(p).unwrap() + 1);
        }
    }
}
