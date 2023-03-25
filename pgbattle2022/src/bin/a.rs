#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, q: usize, p: [(usize, usize); q]}

    let mut res = vec![0; n+2];
    for (l, r) in p {
        res[l] += 1;
        res[r+1] -= 1;
    }
    let mut ans = 0;
    for i in 1..=n {
        if res[i] % 2 == 1 {
            ans += 1;
        }
        res[i+1] += res[i];
    }
    println!("{}", ans);
}
