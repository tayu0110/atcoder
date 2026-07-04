use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: i64, p: [i64; n]}

    let (mut l, mut r) = (-1, m + 1);
    while r - l > 1 {
        // m = p(2k-1)
        let t = (r + l) / 2;
        let mut sum = 0i64;
        for &p in &p {
            let k = t.saturating_add(p) / (2 * p);
            sum = sum.saturating_add(k.saturating_mul(k).saturating_mul(p));
        }
        if sum <= m {
            l = t;
        } else {
            r = t;
        }
    }
    let t = l;
    let mut res = 0;
    let mut sum = 0;
    let mut nt = BinaryHeap::new();
    for (i, &p) in p.iter().enumerate() {
        let k = t.saturating_add(p) / (2 * p);
        sum += k.saturating_mul(k).saturating_mul(p);
        res += k;
        nt.push(Reverse((p.saturating_mul(2 * k + 1), k + 1, i)));
    }
    while let Some(Reverse((np, k, i))) = nt.pop().filter(|v| sum.saturating_add(v.0 .0) <= m) {
        sum += np;
        res += 1;
        nt.push(Reverse((p[i].saturating_mul(2 * k + 1), k + 1, i)));
    }
    println!("{res}")
}
