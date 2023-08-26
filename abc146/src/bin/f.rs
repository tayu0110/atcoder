use itertools::Itertools;
use proconio::*;
use segtree::SegmentTree;

fn main() {
    input! {n: usize, m: usize, s: marker::Chars}

    let mut list = vec![std::usize::MAX; n + 1];
    let mut st = SegmentTree::new(n + 1, (std::usize::MAX, std::usize::MAX), |&l, &r| l.min(r));
    st.set(0, (0, 0));
    for (i, c) in s.into_iter().enumerate().skip(1) {
        if c == '1' {
            continue;
        }
        let (d, prev) = st.foldl(i.saturating_sub(m), i);
        list[i] = prev;
        st.set(i, (d, i));
    }

    let mut res = vec![];
    let mut now = n;
    while now < std::usize::MAX {
        res.push(now);
        now = list[now];
    }
    res.reverse();

    if res[0] != 0 {
        println!("-1")
    } else {
        println!("{}", res.windows(2).map(|v| v[1] - v[0]).join(" "))
    }
}
