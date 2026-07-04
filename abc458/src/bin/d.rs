use ds::{DynamicSortedSequence, MapMonoid};
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = usize;
    type Act = ();
    fn e() -> Self::M {
        0
    }
    fn id() -> Self::Act {}
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(_m: &Self::M, _act: &Self::Act) -> Self::M {
        0
    }
}

fn main() {
    input! {x: usize, q: usize, query: [(usize, usize); q]}

    let mut seq = DynamicSortedSequence::<T>::new();
    seq.insert(x);
    for (a, b) in query {
        seq.insert(a);
        seq.insert(b);
        let len = seq.len();
        println!("{}", seq.get(len / 2).unwrap())
    }
}
