use ds::{DynamicSequence, MapMonoid};
use itertools::Itertools;
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = usize;
    type Act = ();
    fn e() -> Self::M {
        0
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
    input! {n: usize, p: [usize; n]}

    let mut seq = DynamicSequence::<T>::new(0);
    for (i, p) in p.into_iter().enumerate() {
        seq.insert(p - 1, i + 1);
    }

    println!("{}", seq.into_iter().join(" "))
}
