use proconio::*;
use segtree::RangeAddRangeMaximumQuery;
// use segtree::SegmentTree;

const MAX: usize = 200010;

fn solve(n: usize, d: usize, w: usize, mut p: Vec<(usize, usize)>) -> usize {
    p.sort_unstable_by_key(|v| (v.1, v.0));

    let mut st = RangeAddRangeMaximumQuery::new(MAX);
    let (mut l, mut r) = (0, 0);
    let mut res = 0;
    let make_term = |t: usize| ((t as i64 - d as i64 + 1).max(0) as usize, t + 1);
    while l < n {
        let (t, x) = p[l];
        while r < n && p[r].1 < x + w {
            let (left, right) = make_term(p[r].0);
            st.apply_range(left, right, 1);
            r += 1;
        }

        let max = st.all_prod();
        res = res.max(max as usize);
        let (left, right) = make_term(t);
        st.apply_range(left, right, -1);
        l += 1;
    }
    // let mut st = SegmentTree::new(MAX, 0usize, |l, r| l + r);
    // let (mut l, mut r) = (0, 0);
    // let mut res = 0;
    // while l < n {
    //     let (t, x) = p[l];
    //     while r < n && p[r].1 < x + w {
    //         st.update_by(p[r].0, 1, |l, r| l + r);
    //         r += 1;
    //     }

    //     let sum = st.foldl(t, MAX.min(t + d));
    //     res = res.max(sum);
    //     st.update_by(t, 1, |l, r| l - r);
    //     l += 1;
    // }

    res
}

fn main() {
    input! {n: usize, d: usize, w: usize, mut p: [(usize, usize); n]}

    let res = solve(n, d, w, p.clone());

    // {
    //     // reverse left-right
    //     let mut p = p.clone();
    //     p.iter_mut().for_each(|v| v.1 = MAX - v.1);
    //     res = res.max(solve(n, d, w, p));
    // }

    // {
    //     // reverse up-down
    //     let mut p = p.clone();
    //     p.iter_mut().for_each(|v| v.0 = MAX - v.0);
    //     res = res.max(solve(n, d, w, p));
    // }

    // {
    //     // reverse up-down and left-right
    //     let mut p = p.clone();
    //     p.iter_mut().for_each(|v| {
    //         v.0 = MAX - v.0;
    //         v.1 = MAX - v.1;
    //     });
    //     res = res.max(solve(n, d, w, p));
    // }

    println!("{res}")
}
