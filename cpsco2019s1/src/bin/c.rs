use std::{cmp::Reverse, collections::HashSet};

use proconio::*;

fn rec(k: usize, used: usize, now: usize, sum: usize, a: &[usize], memo: &mut HashSet<usize>) {
    if used == k {
        memo.insert(sum);
        return;
    }
    if now == a.len() {
        return;
    }

    rec(k, used, now + 1, sum, a, memo);
    rec(k, used + 1, now + 1, sum + a[now], a, memo);
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut memo = HashSet::new();
    rec(k, 0, 0, 0, &a, &mut memo);

    let mut coins = vec![];
    let mut ten = 1usize;
    for _ in 0..10 {
        coins.push(ten);
        coins.push(ten * 5);
        ten *= 10;
    }
    coins.sort_by_key(|&v| Reverse(v));

    let mut res = std::usize::MAX;
    for mut now in memo {
        let mut used = 0;
        for c in &coins {
            used += now / c;
            now %= c;
        }

        res = res.min(used);
    }

    println!("{}", res)
}
