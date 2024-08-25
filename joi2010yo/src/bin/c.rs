use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut res = vec![false; n];
    res[0] = true;
    for _ in 0..2 {
        let mut new = vec![false; n];
        for i in 0..n {
            if res[i] {
                new[i] = true;
                for &to in &t[i] {
                    new[to] = true;
                }
            }
        }

        res = new;
    }

    println!("{}", res.iter().filter(|&&r| r).count() - 1);
}
