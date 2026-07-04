use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize, e: [(usize, usize); n - 1], c: [usize; k]}

        if let Some(pos) = c.iter().position(|&t| t >= n) {
            println!("{}", (0..n).map(|_| pos + 1).join(" "));
            continue;
        }
        if c.iter().all(|&t| t <= 1) {
            println!("-1");
            continue;
        }

        let mut t = vec![vec![]; n];
        for &(u, v) in &e {
            t[u - 1].push(v - 1);
            t[v - 1].push(u - 1);
        }
    }
}
