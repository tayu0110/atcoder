use proconio::*;

fn main() {
    input! {n: usize, _p: [(usize, usize); n], e: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }
}
