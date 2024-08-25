use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes, t: marker::Bytes}

    let mut dp = vec![vec![usize::MAX; 1 << (n + 2)]; n + 1];
    let now = s
        .iter()
        .rev()
        .map(|&c| (c == b'W') as usize)
        .fold(0, |s, v| (s << 1) | v);
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, now, n)));
    while let Some(Reverse((d, mask, a))) = nt.pop() {
        if dp[a][mask] <= d {
            continue;
        }
        dp[a][mask] = d;

        let b = a + 1;
        for i in 0..n + 1 {
            if i == a || i + 1 == a || i == b || i + 1 == b {
                continue;
            }

            let k = (mask >> i) & 0b11;
            let m = !(0b11 << a);
            let new = (mask & m) + (k << a);
            if dp[i][new] > d + 1 {
                nt.push(Reverse((d + 1, new, i)));
            }
        }
    }

    let dest = t
        .iter()
        .rev()
        .map(|&c| (c == b'W') as usize)
        .fold(0, |s, v| (s << 1) | v);
    let mut res = usize::MAX;
    for i in 0..1 << (n + 2) {
        if (i & ((1 << n) - 1)) == dest {
            res = res.min(dp[n][i]);
        }
    }
    println!("{}", res as i64)
}
