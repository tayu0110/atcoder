#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [usize; n], t: [usize; n]}

    let mut res = vec![std::usize::MAX; n];
    let mut nt = t
        .into_iter()
        .enumerate()
        .map(|(i, t)| std::cmp::Reverse((t, i)))
        .collect::<std::collections::BinaryHeap<_>>();
    while let Some(std::cmp::Reverse((t, i))) = nt.pop() {
        if res[i] != std::usize::MAX {
            continue;
        }
        res[i] = t;

        nt.push(std::cmp::Reverse((t + s[i], (i + 1) % n)));
    }

    for res in res {
        println!("{}", res);
    }
}
