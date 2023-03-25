#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};
use std::collections::{HashMap, HashSet};

fn dfs(now: usize, par: usize, memo: &mut HashMap<usize, usize>, pending: &mut HashSet<usize>, map: &HashMap<usize, Vec<usize>>) -> usize {
    if memo.contains_key(&now) {
        return *memo.get(&now).unwrap();
    }

    let mut res = now;
    pending.insert(now);
    if !map.contains_key(&now) {
        memo.insert(now, now);
        return now;
    }
    let v = map.get(&now).unwrap();
    for to in v {
        if to == &par {
            continue;
        }
        if !pending.contains(to) {
            res = std::cmp::max(res, dfs(*to, now, memo, pending, map));
        }
    }

    memo.insert(now, res);

    pending.remove(&now);

    res
}

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut map = HashMap::new();
    for (a, b) in p {
        map.entry(a).or_insert(vec![]).push(b);
        map.entry(b).or_insert(vec![]).push(a);
    }

    let mut memo = HashMap::new();
    let mut pending = HashSet::new();
    dfs(1, 1, &mut memo, &mut pending, &map);

    println!("{}", memo.get(&1).unwrap());
}
