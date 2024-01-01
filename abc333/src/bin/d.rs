use proconio::*;

fn dfs(now: usize, par: usize, t: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    let mut max = 0;
    for &to in &t[now] {
        if to == par {
            continue;
        }

        let r = dfs(to, now, t);
        sum += r;
        max = max.max(r);
    }

    let mut res = sum + 1;
    if par == usize::MAX {
        res -= max;
    }
    res
}

fn main() {
    input! {n: usize, e: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    println!("{}", dfs(0, usize::MAX, &t))
}
