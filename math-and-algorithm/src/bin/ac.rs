use proconio::*;
use segtree::SegmentTree;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut st = SegmentTree::new(n, 0usize, |&l, &r| l.max(r));
    for (i, a) in a.into_iter().enumerate() {
        let b = if i > 0 { st.foldl(0, i - 1) } else { 0 };
        st.set(i, a + b);
    }

    println!("{}", st.foldl(0, n))
}
