use std::collections::BTreeSet;

use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    type M = i64;
    fn id() -> Self::M {
        i64::MIN
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut pos = vec![0; n];
    for (i, p) in p.iter().enumerate() {
        pos[p - 1] = i;
    }

    let mut ret = 0;
    let mut pst = SegmentTree::<T>::new(n);
    let mut nst = SegmentTree::<T>::new(n);
    let mut unused = (-1..=n as i64).collect::<BTreeSet<_>>();
    for pos in pos {
        unused.remove(&(pos as i64));

        let mut tmp = 0;
        if let Some(prev) = unused
            .range(..pos as i64)
            .next_back()
            .map(|prev| prev + 1)
            .filter(|&prev| prev < pos as i64)
        {
            let ret = pos as i64 + pst.fold(prev as usize..pos);
            // eprintln!("prev: {prev}, ret: {ret}");
            tmp = tmp.max(ret);
        }

        if let Some(&next) = unused
            .range(pos as i64..)
            .next()
            .filter(|&&next| pos as i64 + 1 < next)
        {
            let ret = nst.fold(pos + 1..next as usize) - pos as i64;
            // eprintln!("next: {next}, ret: {ret}");
            tmp = tmp.max(ret);
        }

        pst.set(pos, tmp - pos as i64);
        nst.set(pos, pos as i64 + tmp);
        // eprintln!("pos: {pos}, tmp: {tmp}");

        ret = ret.max(tmp);
    }

    println!("{ret}")
}
