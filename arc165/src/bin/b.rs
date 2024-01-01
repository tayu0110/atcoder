use itertools::Itertools;
use proconio::*;
use segtree::SegmentTree;

fn main() {
    input! {n: usize, k: usize, mut p: [usize; n]}
    p.iter_mut().for_each(|v| *v -= 1);

    let mut st = SegmentTree::new(n, usize::MAX, |&l, &r| l.min(r));
    let mut res = 0;
    let mut max = 0;

    let (mut l, mut r) = (0, 0);
    while l + k <= n {
        while r < n && r - l < k && st.foldl(p[r], n) == usize::MAX {
            st.set(p[r], r);
            r += 1;
        }

        if r - l == k {
            p[l..r].sort();
            println!("{}", p.iter().map(|v| v + 1).join(" "));
            return;
        }

        let t = st.foldl(p[r], n);
        if t > max {
            max = t;
            res = l;
        }

        st.set(p[l], usize::MAX);
        l += 1;
        while l < r && st.foldl(p[r], n) < usize::MAX {
            st.set(p[l], usize::MAX);
            l += 1;
        }
    }

    p[res..res + k].sort();
    println!("{}", p.iter().map(|v| v + 1).join(" "))
}
