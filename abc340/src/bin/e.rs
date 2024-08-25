use itertools::Itertools;
use proconio::*;
use segtree::RangeAddRangeSumQuery;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut st = RangeAddRangeSumQuery::new(n * 2);
    for (i, a) in a.into_iter().enumerate() {
        st.set(i, (a, 1));
    }

    for b in b {
        let mut ball = st.get(b).0 + st.get(b + n).0;
        st.set(b, (0, 1));
        st.set(b + n, (0, 1));

        if ball > n {
            st.apply_range(0, n, ball / n);
            ball %= n;
        }
        st.apply_range(b + 1, b + ball + 1, 1);
    }

    let mut res = vec![];
    for i in 0..n {
        res.push(st.get(i).0 + st.get(i + n).0);
    }

    println!("{}", res.iter().join(" "))
}
