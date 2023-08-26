use proconio::*;

fn main() {
    input! {n: usize, s: usize, t: usize, e: [(usize, usize); n-1]}

    let mut g = vec![vec![]; n];
    for (u, v) in e {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }
}
