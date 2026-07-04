use proconio::*;

fn solve(now: usize, par: usize, checked: &mut [bool], t: &[Vec<usize>]) -> usize {
    checked[now] = true;

    let mut res = 0;
    for &to in &t[now] {
        if to == par {
            continue;
        }
        if t[to].len() == 3 {
            res += solve(to, now, checked, t);
        } else if t[to].len() == 2 {
            res += 1;
        }
    }
    res
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut res = 0;
    let mut checked = vec![false; n];
    for i in 0..n {
        if !checked[i] && t[i].len() == 3 {
            let leaf = solve(i, i, &mut checked, &t);
            res += leaf * (leaf - 1) / 2;
        }
        checked[i] = true;
    }

    println!("{res}")
}
