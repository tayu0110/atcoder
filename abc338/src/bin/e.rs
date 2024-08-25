use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UsizeMax;
impl Monoid for UsizeMax {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, mut e: [(usize, usize); n]}

    for i in 0..n {
        let (mut s, mut t) = e[i];
        if s > t {
            (s, t) = (t, s);
        }
        if t - s > s + n * 2 - t {
            e[i] = (t, s + n * 2);
        } else {
            e[i] = (s, t);
        }
    }

    let mut f = e.clone();
    f.iter_mut().for_each(|(s, t)| {
        *s += 2 * n;
        *t += 2 * n
    });
    e.extend(f);
    let mut t = vec![vec![]; 6 * n + 10];
    for (v, w) in e {
        t[v].push(w);
    }

    let mut st = SegmentTree::<UsizeMax>::new(6 * n + 10);
    for i in 0..6 * n + 5 {
        if !t[i].is_empty() {
            t[i].sort_unstable();
            st.set(i, *t[i].last().unwrap());
        }
    }

    for i in 0..6 * n + 5 {
        if !t[i].is_empty() {
            let close = t[i][0];
            if st.fold(i + 1..close) > close {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
