use proconio::*;
use segtree::{Monoid, SegmentTree};

struct T;
impl Monoid for T {
    type M = u32;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        *l.max(r)
    }
}

fn main() {
    input! {n: usize, mut b: [u32; n]}

    let mut c = b.clone();
    c.sort_unstable();
    c.dedup();

    for b in b.iter_mut() {
        *b = c.partition_point(|&c| c < *b) as u32;
    }

    let mut memo = [
        SegmentTree::<T>::new(c.len()),
        SegmentTree::<T>::new(c.len()),
    ];
    for b in b {
        let m1 = memo[0].fold(b as usize + 1..);
        let m2 = memo[1].fold(..b as usize);
        memo[0].update_by(b as usize, |&old| old.max(m2 + 1));
        memo[1].update_by(b as usize, |&old| old.max(m1 + 1));
    }

    println!("{}", memo.iter().map(|m| m.fold(..)).max().unwrap());
}
