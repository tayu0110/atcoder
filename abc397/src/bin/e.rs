use proconio::*;

fn dfs(now: usize, par: usize, k: usize, t: &[Vec<usize>]) -> usize {
    let mut children = vec![];
    for &to in &t[now] {
        if to == par {
            continue;
        }

        let r = dfs(to, now, k, t);
        if r == usize::MAX {
            return usize::MAX;
        }
        if r > 0 {
            children.push(r);
        }
    }

    if children.is_empty() {
        return 1;
    }

    if children.len() > 2 {
        return usize::MAX;
    }

    if children.len() == 1 {
        let s = children[0];
        if s + 1 == k {
            return 0;
        } else {
            return s + 1;
        }
    }

    let s = children[0];
    let t = children[1];
    if s + t + 1 == k {
        0
    } else {
        usize::MAX
    }
}

fn main() {
    input! {n: usize, k: usize, e: [(usize, usize); n * k - 1]}

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut t = vec![vec![]; n * k];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    if dfs(0, 0, k, &t) == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
