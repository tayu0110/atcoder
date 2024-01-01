use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}

    p.iter_mut().for_each(|(v, w)| *w += *v);
    p.sort_unstable_by_key(|&v| Reverse(v.0));

    let mut res = 0;
    let mut nt = BinaryHeap::new();
    let mut now = 0;
    while let Some((t, d)) = p.pop() {
        nt.push(Reverse(d));
        now = t.max(now);
        let mut next = usize::MAX;
        while let Some((nxt, d)) = p.pop() {
            if t == nxt {
                nt.push(Reverse(d));
            } else {
                next = nxt;
                p.push((nxt, d));
                break;
            }
        }

        while let Some(Reverse(d)) = nt.pop() {
            if d < now {
                continue;
            }
            if now <= d {
                res += 1;
                now += 1;
            }

            if now == next {
                break;
            }
        }
    }

    println!("{res}");
}
