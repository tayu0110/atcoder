use proconio::*;
use segtree::{Monoid, SegmentTree};

struct I64Max;

impl Monoid for I64Max {
    type M = i64;
    fn id() -> Self::M {
        i64::MIN
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).max(*r)
    }
}

fn main() {
    input! {n: usize, c: i64, m: usize, p: [(usize, i64); m]}

    let mut dp = vec![i64::MIN; n];
    dp[0] = 0;
    let mut add_st = SegmentTree::<I64Max>::new(n);
    let mut sub_st = SegmentTree::<I64Max>::new(n);

    add_st.set(0, 0);
    sub_st.set(0, 0);

    for (t, p) in p {
        let a = add_st.fold(t..).saturating_add(c * (t as i64 - 1));
        let b = sub_st.fold(..t).saturating_sub(c * (t as i64 - 1));

        dp[t - 1] = dp[t - 1].max(a.max(b) + p);
        add_st.update_by(t - 1, |&m| m.max(dp[t - 1] - c * (t as i64 - 1)));
        sub_st.update_by(t - 1, |&m| m.max(dp[t - 1] + c * (t as i64 - 1)));
    }

    println!("{}", dp.iter().max().unwrap())
}
