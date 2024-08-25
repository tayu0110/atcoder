use proconio::*;
use segtree::{DynamicSegmentTree, Monoid};

struct IsizeAdd;

impl Monoid for IsizeAdd {
    type M = isize;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
}

fn main() {
    input! {n: usize, k: isize, a: [isize; n]}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let (mut l, mut r) = (-1isize, 1000_000_000_000_000isize);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut st = DynamicSegmentTree::<IsizeAdd>::new(-1000000000000000..1000000000000000);
        for &c in &cum {
            let now = st.fold(c..c + 1);
            st.set(c, now + 1);
        }

        let mut cnt = 0;
        for &c in &cum {
            let now = st.fold(c..c + 1);
            st.set(c, now - 1);
            // |cum[j] - c| <= m
            // cum[j] >= c -> cum[j] - c <= m
            // cum[j] <  c -> c - cum[j] <= m
            cnt += st.fold(c - m..=m + c);
        }

        if cnt < k {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", r);
}
