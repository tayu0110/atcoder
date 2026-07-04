use proconio::*;

fn main() {
    input! {n: usize, x: usize, k: i64, e: [(usize, i64, usize); n]}

    let mut t = vec![vec![]; n];
    for (p, u, c) in e {
        t[c - 1].push((p, u));
    }

    let mut memo = vec![-1; x + 1];
    memo[0] = 0;
    for t in t {
        if !t.is_empty() {
            let mut new = vec![-1; x + 1];
            for (p, u) in t {
                for i in (0..x).rev() {
                    if memo[i] >= 0 && i + p <= x {
                        new[i + p] = new[i + p].max(memo[i] + k + u);
                    }
                    if new[i] >= 0 && i + p <= x {
                        new[i + p] = new[i + p].max(new[i] + u);
                    }
                }
            }
            for i in 0..=x {
                memo[i] = memo[i].max(new[i]);
            }
        }
    }

    println!("{}", memo.into_iter().max().unwrap())
}
