use std::collections::VecDeque;

use itertools::Itertools;
use math::MathInt;
use proconio::*;
use rustc_hash::{FxHashMap, FxHashSet};

fn solve(
    now: usize,
    pairs: &FxHashMap<usize, usize>,
    res: &mut VecDeque<usize>,
    memo: &mut FxHashSet<usize>,
) -> bool {
    if now == 1 {
        return true;
    }

    if pairs.get(&now) == Some(&now) {
        res.push_back(now);
        return true;
    }

    if memo.contains(&now) {
        return false;
    }

    for (&k, &v) in pairs.iter() {
        if now % (k * v) == 0 {
            if solve(now / k / v, pairs, res, memo) {
                res.push_front(k);
                res.push_back(v);
                return true;
            }
        }
    }

    memo.insert(now);
    false
}

fn main() {
    input! {n: usize}

    if n == 1 {
        println!("1");
        return;
    }

    let divisors = n.divisors();
    let mut set = divisors.iter().cloned().collect::<FxHashSet<usize>>();
    let mut pairs = FxHashMap::default();
    for &d in &divisors {
        if d == 1 {
            set.remove(&d);
            continue;
        }

        let s = d.to_string();
        if s.chars().any(|c| c == '0') {
            set.remove(&d);
            continue;
        }
        let s = s
            .chars()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        if !set.contains(&s) {
            set.remove(&d);
        }

        if d <= s {
            pairs.insert(d, s);
        }
    }

    let mut res = VecDeque::new();
    let mut memo = FxHashSet::default();
    if solve(n, &pairs, &mut res, &mut memo) {
        println!("{}", res.iter().join("*"));
    } else {
        println!("-1");
    }
}
