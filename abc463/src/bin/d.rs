use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    type M = (i64, usize);
    fn id() -> Self::M {
        (i64::MAX, usize::MAX)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn main() {
    input! {n: usize, k: usize, mut p: [(i64, i64); n]}

    p.sort_unstable();
    let mut seg = SegmentTree::<T>::new(n);
    for (i, &(_, r)) in p.iter().enumerate() {
        seg.set(i, (r, i));
    }

    let (mut l, mut r) = (-1, 1000_000_010);
    while r - l > 1 {
        let m = (r + l) / 2;

        let (_r, i) = seg.fold(..);
        // eprintln!("_r: {_r}, i: {i}");
        let mut now = i;
        let mut cnt = 0;
        while now < n {
            cnt += 1;
            let r = p[now].1;
            let next = r + m;
            let pos = p.partition_point(|p| p.0 < next);
            let (_r, next) = seg.fold(pos..);
            // eprintln!("_r: {_r}, next: {next}");
            now = next;
        }

        // eprintln!("m: {m}, cnt: {cnt}, now: {now}");
        if cnt >= k {
            l = m;
        } else {
            r = m;
        }
    }

    if l == 0 {
        println!("-1")
    } else {
        println!("{}", l)
    }
}
