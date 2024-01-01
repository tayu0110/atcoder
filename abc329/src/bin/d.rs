use proconio::*;
use segtree::SegmentTree;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut st = SegmentTree::new(n, (0usize, 0usize), |&l, &r| l.max(r));
    for i in 0..n {
        st.set(i, (0, usize::MAX - i));
    }

    for a in a {
        st.update_by(a - 1, (1, 0), |&l, &r| (l.0 + r.0, l.1));
        let (_, res) = st.foldl(0, n);
        println!("{}", usize::MAX - res + 1);
    }
}
