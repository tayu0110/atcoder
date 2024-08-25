use proconio::*;
use segtree::{Monoid, SegmentTree};

struct I64Min;

impl Monoid for I64Min {
    type M = i64;
    fn id() -> Self::M {
        i64::MAX
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn binary_search(t: usize, p: &[usize]) -> usize {
    let (mut l, mut r) = (-1, p.len() as i32);
    while r - l > 1 {
        let m = (r + l) / 2;
        if p[m as usize] < t {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut q = p
        .iter()
        .cloned()
        .flat_map(|(l, r)| vec![l - 1, r])
        .collect::<Vec<_>>();
    q.insert(0, 0);
    q.sort();
    q.dedup();

    let len = q.len();

    let mut st1 = SegmentTree::<I64Min>::new(len);
    let mut st2 = SegmentTree::<I64Min>::new(len);

    st1.set(0, 0);
    st2.set(0, 0);

    let mut res = 0;
    for (l, r) in p {
        let (li, ri) = (binary_search(l - 1, &q), binary_search(r, &q));

        let a = st1.fold(li..ri + 1);
        let b = st2.fold(0..li) + l as i64 - 1;
        let c = a.min(b);

        st1.set(ri, c);
        st2.set(ri, c - r as i64);
        res = res.max(r as i64 - c);
    }

    println!("{}", res)
}
