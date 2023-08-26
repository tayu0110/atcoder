use proconio::*;
use segtree::SegmentTree;

fn binary_search(t: usize, p: &[usize]) -> usize {
    let (mut l, mut r) = (-1, p.len() as i32);
    while r - l > 1 {
        let m = (r + l) / 2;
        if p[m as usize] < t {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut q = p
        .iter()
        .cloned()
        .map(|(l, r)| vec![l - 1, r])
        .flatten()
        .collect::<Vec<_>>();
    q.insert(0, 0);
    q.sort();
    q.dedup();

    let len = q.len();

    let mut st1 = SegmentTree::new(len, std::i64::MAX >> 10, |&l, &r| l.min(r));
    let mut st2 = SegmentTree::new(len, std::i64::MAX >> 10, |&l, &r| l.min(r));

    st1.set(0, 0);
    st2.set(0, 0);

    let mut res = 0;
    for (l, r) in p {
        let (li, ri) = (binary_search(l - 1, &q), binary_search(r, &q));

        let a = st1.foldl(li, ri + 1);
        let b = st2.foldl(0, li) + l as i64 - 1;
        let c = a.min(b);

        st1.set(ri, c);
        st2.set(ri, c - r as i64);
        res = res.max(r as i64 - c);
    }

    println!("{}", res)
}
