#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: Chars}

    let mut res = 0;
    for i in 1..n {
        let a = s[..i].into_iter().collect::<std::collections::HashSet<_>>();
        let b = s[i..].into_iter().collect::<std::collections::HashSet<_>>();

        let c = a.intersection(&b).count();
        res = std::cmp::max(res, c);
    }

    println!("{}", res);
}
