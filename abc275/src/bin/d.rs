#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn rec(now: usize, memo: &mut std::collections::HashMap<usize, usize>) -> usize {
    if now == 0 {
        return 1;
    }
    if memo.contains_key(&now) {
        return *memo.get(&now).unwrap();
    }

    *memo.entry(now).or_insert(0) = rec(now / 2, memo) + rec(now / 3, memo);
    *memo.get(&now).unwrap()
}

#[fastout]
fn main() {
    input! {n: usize}

    let mut map = std::collections::HashMap::new();

    println!("{}", rec(n, &mut map));
}
