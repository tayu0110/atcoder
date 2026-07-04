use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1], q: usize, query: [usize; q]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }
}
