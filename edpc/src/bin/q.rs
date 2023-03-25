use proconio::input;
use segtree::SegmentTree;

fn main() {
    input! {n: usize, h: [usize; n], a: [usize; n]}

    let mut st = SegmentTree::new(n, 0, |&l, &r| std::cmp::max(l, r));
    for (h, a) in h.into_iter().zip(a.into_iter()) {
        let max = st.foldl(0, h - 1);
        st.set(h - 1, max + a);
    }

    println!("{}", st.foldl(0, n))
}
