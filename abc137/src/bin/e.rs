#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, p: i64, mut e: [(usize, usize, i64); m]}
    e.iter_mut().for_each(|(_, _, c)| *c = p - *c);

    const MIN: i64 = std::i64::MIN / 16;

    let mut dist = vec![std::i64::MAX; n];
    dist[0] = 0;
    for turn in 0..2 * n + 1 {
        for &(a, b, c) in &e {
            if dist[a - 1] != std::i64::MAX {
                if turn >= n {
                    if dist[b - 1] > dist[a - 1] + c {
                        dist[b - 1] = MIN;
                    }
                } else {
                    dist[b - 1] = std::cmp::min(dist[b - 1], dist[a - 1] + c);
                }
            }
        }
    }

    if dist[n - 1] == MIN {
        println!("-1");
    } else {
        println!("{}", std::cmp::max(0, -dist[n - 1]));
    }
}
