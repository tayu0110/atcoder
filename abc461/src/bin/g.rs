use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut d = vec![(0, 0); n];
    for i in 0..n {
        d[i] = (t[i].len(), i);
    }
    d.sort_unstable();

    let mut r = vec![0; n];
    let mut ret = 0;
    for (_, i) in d {
        if t[i].iter().all(|&n| r[n] == 0) {
            ret += 2026;
            r[i] = 2026;
        }
    }
    println!("{ret}")
}
