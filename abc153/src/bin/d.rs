#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn rec(now: usize, memo: &mut std::collections::HashMap<usize, usize>) -> usize {
    if now == 1 {
        return 1;
    }
    if memo.contains_key(&now) {
        return *memo.get(&now).unwrap();
    }

    let res = rec(now / 2, memo);
    *memo.entry(now / 2).or_insert(0) = res;
    res * 2 + 1
}

fn main() {
    input! {h: usize}

    let mut memo = std::collections::HashMap::new();
    println!("{}", rec(h, &mut memo));
}
