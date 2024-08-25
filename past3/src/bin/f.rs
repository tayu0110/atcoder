#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [Chars; n]}

    let mut res = vec![];
    let (mut l, mut r) = (0, n - 1);
    while l <= r {
        let set = a[l]
            .iter().copied()
            .collect::<std::collections::HashSet<_>>();
        for i in 0..n {
            if set.contains(&a[r][i]) {
                res.push(a[r][i]);
                break;
            }
        }

        l += 1;
        r = r.saturating_sub(1);
    }

    let mut res2 = res.clone();
    if n % 2 == 1 && !res2.is_empty() {
        res2.pop().unwrap();
    }
    res2.reverse();
    let res = [res, res2].concat().into_iter().collect::<String>();
    if res.len() != n {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
