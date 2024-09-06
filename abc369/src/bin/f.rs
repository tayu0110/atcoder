use itertools::Itertools;
use proconio::*;
use segtree::{Monoid, SegmentTree};

struct T;

impl Monoid for T {
    type M = (usize, usize, usize);
    fn id() -> Self::M {
        (0, 0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {h: usize, w: usize, n: usize, coins: [(usize, usize); n]}

    let mut t = vec![vec![]; h + 1];
    t[1].push(1);
    for (r, c) in coins {
        t[r].push(c);
    }
    t[h].push(w);

    let mut memo = SegmentTree::<T>::new(w + 1);
    memo.set(1, (0, 1, 0));
    let mut prev = vec![vec![]; h + 1];
    for i in 1..=h {
        t[i].sort_unstable();
        let l = t[i].len();
        for j in 0..l {
            let c = t[i][j];
            let (max, pi, pj) = memo.fold(..=c);
            prev[i].push((pi, pj));
            memo.update_by(c, |prev| {
                if prev.0 < max + 1 {
                    (max + 1, i, j)
                } else {
                    *prev
                }
            });
        }
    }

    println!("{}", memo.fold(..).0 - 2);
    let mut stack = vec![];
    let mut now = (h, t[h].len() - 1);
    while now != (1, 0) {
        let (r, j) = now;
        let (pr, pj) = prev[r][j];

        let c = t[r][j];
        let pc = t[pr][pj];
        stack.resize(stack.len() + c.abs_diff(pc), 'R');
        stack.resize(stack.len() + r.abs_diff(pr), 'D');
        now = (pr, pj);
    }
    assert!(stack.len() == h + w - 2);
    stack.reverse();
    println!("{}", stack.iter().join(""))
}
