use std::cmp::min_by_key;

use ds::{DynamicSortedSequence, MapMonoid};
use itertools::Itertools;
use proconio::*;

struct T;

impl MapMonoid for T {
    type M = (usize, usize);
    type Act = ();

    fn e() -> Self::M {
        (usize::MAX, usize::MAX)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        min_by_key(*l, *r, |l| l.1)
    }

    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}

    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut seq = DynamicSortedSequence::<T>::new();
    for (i, &b) in b.iter().enumerate() {
        seq.insert((b, i));
    }

    let mut rem = b.clone();
    for (j, a) in a.into_iter().enumerate() {
        let Some(&(c, i)) = seq.range((a, 0)..).next() else {
            println!("No");
            println!("{}", j + 1);
            return;
        };

        let index = seq.first_index_of(&(c, i)).unwrap();
        let (c, i) = seq.fold(index..);

        seq.remove_once(&(c, i));
        rem[i] -= a;
        seq.insert((rem[i], i));
    }

    println!("Yes");
    println!("{}", b.into_iter().zip(rem).map(|(b, r)| b - r).join(" "));
}
