use proconio::*;
use segtree::{Monoid, SegmentTree};

struct UiszeMAX;
impl Monoid for UiszeMAX {
    type M = usize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

struct UsizeMin;
impl Monoid for UsizeMin {
    type M = usize;
    fn id() -> Self::M {
        usize::MAX
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).min(*r)
    }
}

fn main() {
    input! {n: usize, q: usize, h: [usize; n], query: [(usize, usize); q]}

    let mut memo = vec![0; n + 1];
    let mut next = vec![n; n];
    {
        let mut st = SegmentTree::<UsizeMin>::new(n + 10);
        for (i, &h) in h.iter().enumerate().rev() {
            let min = st.fold(h..);
            if min != usize::MAX {
                memo[i] = memo[min] + 1;
                next[i] = min;
            }
            st.set(h, i);
        }
    }

    const MAX: usize = 20;
    let mut doubling = vec![vec![n; n]; MAX];
    doubling[0] = next.clone();
    for i in 0..MAX - 1 {
        for j in 0..n {
            if doubling[i][j] != n {
                doubling[i + 1][j] = doubling[i][doubling[i][j]];
            }
        }
    }

    let mut st = SegmentTree::<UiszeMAX>::new(n);
    for (i, &h) in h.iter().enumerate() {
        st.set(i, h);
    }

    for (l, r) in query {
        if r == n || next[r - 1] == n {
            println!("0");
            continue;
        }

        let (l, r) = (l - 1, r - 1);
        let max = st.fold(l + 1..=r);
        let (mut lh, mut rh) = (0, n);
        while rh - lh > 1 {
            let mid = (rh + lh) / 2;
            let mut mask = mid;
            let mut now = r;
            for i in (0..MAX).rev() {
                if mask >= 1 << i {
                    mask -= 1 << i;
                    if now < n {
                        now = doubling[i][now];
                    }
                }
            }

            if now == n {
                rh = mid;
            } else if h[now] <= max {
                lh = mid;
            } else {
                rh = mid;
            }
        }

        let mut mask = rh;
        let mut now = r;
        for i in (0..MAX).rev() {
            if mask >= 1 << i {
                mask -= 1 << i;
                if now < n {
                    now = doubling[i][now];
                }
            }
        }
        if now == n {
            println!("0")
        } else {
            println!("{}", memo[now] + 1);
        }
    }
}
