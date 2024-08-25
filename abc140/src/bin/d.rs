use std::cmp::Reverse;

use proconio::*;

fn solve(k: usize, c: Vec<u8>) -> usize {
    let l = c.iter().filter(|&&c| c == b'L').count();
    let mut t = vec![];
    for c in c {
        match t.last_mut() {
            Some((pc, cnt)) if *pc == c => *cnt += 1,
            _ => t.push((c, 1)),
        }
    }

    let mut rem = t
        .into_iter()
        .filter_map(|(c, cnt)| (c == b'R').then_some(cnt))
        .collect::<Vec<_>>();
    rem.sort_unstable_by_key(|c| Reverse(*c));

    l + rem.into_iter().take(k).sum::<usize>()
}

fn main() {
    input! {_: usize, k: usize, s: marker::Bytes}

    let res = solve(k, s.clone());
    println!(
        "{}",
        res.max(solve(
            k,
            s.into_iter().rev().map(|c| c ^ b'L' ^ b'R').collect()
        ))
    )
}
