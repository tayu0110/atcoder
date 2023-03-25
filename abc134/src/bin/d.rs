#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, mut a: [u8; n]}
    a.insert(0, 0);

    let mut res = vec![0; n + 1];
    for i in (1..=n).rev() {
        let mut sum = 0;
        for j in (1..=n).take_while(|&j| i * j <= n) {
            sum ^= res[i * j];
        }

        if sum % 2 != a[i] {
            res[i] = 1;
        }
    }

    let res = res
        .into_iter()
        .enumerate()
        .filter(|(_, r)| *r == 1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    println!("{}", res.len());
    if !res.is_empty() {
        println!("{}", res.iter().join(" "));
    }
}
