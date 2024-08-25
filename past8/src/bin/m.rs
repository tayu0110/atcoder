use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, i64); m]}

    let mut t = vec![vec![]; n];
    for &(a, b, c) in &e {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut start = 0;
    for _ in 0..2 {
        let mut d = vec![i64::MAX; n];
        d[0] = start;
        let mut parity = vec![0; n];
        for i in 0..n {
            for &(to, c) in &t[i] {
                if d[to] == i64::MAX {
                    d[to] = c - d[i];
                    parity[to] = parity[i] ^ 1;
                } else if d[to] != c - d[i] {
                    println!("-1");
                    return;
                } else {
                    assert_ne!(parity[i], parity[to]);
                }
            }
        }

        if d.iter().all(|&d| d >= 0) {
            println!("{}", d.iter().join("\n"));
            return;
        }

        if (0..n).filter(|&i| parity[i] == 1).any(|i| d[i] < 0) {
            println!("-1");
            return;
        }

        start = -*d.iter().min().unwrap();
    }

    println!("-1");
}
