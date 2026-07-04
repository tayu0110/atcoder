use std::fmt::Write as _;

use ds::{Monoid, SegmentTree};
use proconio::*;

struct Max;
impl Monoid for Max {
    type M = i64;
    fn id() -> Self::M {
        i64::MIN
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, q: usize, e: [(i64, i64); n]}

    let mut sta = SegmentTree::<Max>::new(n);
    let mut stb = SegmentTree::<Max>::new(n);
    let mut stc = SegmentTree::<Max>::new(n);
    let mut std = SegmentTree::<Max>::new(n);
    for (i, &(x, y)) in e.iter().enumerate() {
        sta.set(i, x + y);
        stb.set(i, x - y);
        stc.set(i, -x - y);
        std.set(i, -x + y);
    }

    let mut buf = String::new();
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {i: usize, x: i64, y: i64}
            sta.set(i - 1, x + y);
            stb.set(i - 1, x - y);
            stc.set(i - 1, -x - y);
            std.set(i - 1, -x + y);
        } else {
            input! {l: usize, r: usize, x: i64, y: i64}

            let mut ret = 0;
            ret = ret.max(sta.fold(l - 1..r) - x - y);
            ret = ret.max(stb.fold(l - 1..r) - x + y);
            ret = ret.max(stc.fold(l - 1..r) + x + y);
            ret = ret.max(std.fold(l - 1..r) + x - y);
            writeln!(buf, "{ret}").unwrap();
        }
    }

    print!("{buf}")
}
