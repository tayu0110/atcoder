#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, mut a: [usize; m]}

    let mut next = vec![std::usize::MAX; n + 1];
    for i in 1..=n {
        if a.contains(&i) {
            next[i] = i + 1;
        }
    }

    let mut res = vec![];
    let mut used = vec![false; n + 1];
    for i in 1..=n {
        if used[i] {
            continue;
        }
        let mut buf = vec![i];
        let mut now = i;
        while next[now] != std::usize::MAX {
            buf.push(next[now]);
            now = next[now];
        }

        while let Some(r) = buf.pop() {
            used[r] = true;
            res.push(r);
        }
    }

    println!("{}", res.iter().join(" "))
}
