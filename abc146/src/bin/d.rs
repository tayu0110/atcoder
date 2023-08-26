use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, par: usize, res: &mut [usize], t: &Vec<Vec<(usize, usize)>>) {
    let mut ban = std::usize::MAX;
    if now != par {
        for &(to, n) in &t[now] {
            if to == par {
                ban = res[n];
                break;
            }
        }
    }

    let mut cnt = 1;
    for &(to, n) in &t[now] {
        if to == par {
            continue;
        }
        if cnt == ban {
            cnt += 1;
        }
        res[n] = cnt;
        dfs(to, now, res, t);
        cnt += 1;
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (i, (a, b)) in e.into_iter().enumerate() {
        t[a - 1].push((b - 1, i));
        t[b - 1].push((a - 1, i));
    }

    let mut res = vec![std::usize::MAX; n - 1];
    dfs(0, 0, &mut res, &t);
    println!("{}", res.iter().max().unwrap());
    println!("{}", res.iter().join("\n"))
}
