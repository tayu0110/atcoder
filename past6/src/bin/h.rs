use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, q: usize, e: [(usize, usize); n]}

    let (mut need, mut not_need): (Vec<_>, Vec<_>) = e.into_iter().partition(|&(_, t)| t == 1);

    not_need.sort_unstable();
    let mut sum = not_need.iter().take(m).map(|v| v.0).sum::<usize>();
    let mut buf = not_need
        .iter()
        .take(m)
        .map(|&(p, _)| p)
        .collect::<BinaryHeap<_>>();
    let mut res = usize::MAX;
    if buf.len() == m {
        res = sum;
    }

    need.sort_unstable_by_key(|v| Reverse(v.0));
    let mut rem = 0;
    while let Some((p, _)) = need.pop() {
        if rem == 0 {
            sum += q;
            rem = k;
        }
        rem -= 1;
        sum += p;
        buf.push(p);

        if buf.len() > m {
            let t = buf.pop().unwrap();
            sum -= t;
        }

        if buf.len() == m {
            res = res.min(sum);
        }
    }

    println!("{res}")
}
