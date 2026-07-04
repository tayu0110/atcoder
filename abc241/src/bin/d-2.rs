use ds::{DynamicSortedSequence, MapMonoid};
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
    input! {q: usize}

    let mut seq = DynamicSortedSequence::<T>::new();
    let mut res = vec![];
    for _ in 0..q {
        input! {ty: u8}
        if ty == 1 {
            input! {x: usize}
            seq.insert(x);
        } else if ty == 2 {
            input! {x: usize, k: usize}
            res.push(*seq.range(..=x).nth_back(k - 1).unwrap_or(&usize::MAX) as i64)
        } else {
            input! {x: usize, k: usize}
            res.push(*seq.range(x..).nth(k - 1).unwrap_or(&usize::MAX) as i64)
        }
    }

    println!("{}", res.iter().join("\n"))
}
