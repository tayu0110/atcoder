use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {mut h: usize, mut w: usize, n: usize, p: [(usize, usize); n]}

    let mut a = p
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (h, w))| (h, w, i))
        .collect::<BinaryHeap<_>>();
    let mut b = p
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (h, w))| (w, h, i))
        .collect::<BinaryHeap<_>>();

    let mut used = vec![false; n];
    let mut ret = vec![(0, 0); n];
    let mut rem = n;
    let (mut u, mut l) = (1, 1);
    while rem != 0 {
        while let Some(&(_, nw, i)) = a.peek().filter(|&&(nh, _, _)| h == nh) {
            a.pop();
            if !used[i] {
                used[i] = true;
                ret[i] = (u, l);
                w -= nw;
                l += nw;
                rem -= 1;
            }
        }
        while let Some(&(_, nh, i)) = b.peek().filter(|&&(nw, _, _)| w == nw) {
            b.pop();
            if !used[i] {
                used[i] = true;
                ret[i] = (u, l);
                h -= nh;
                u += nh;
                rem -= 1;
            }
        }
    }

    for (h, w) in ret {
        println!("{} {}", h, w);
    }
}
