use proconio::*;

fn dfs(now: usize, par: usize, perm: &mut Vec<usize>, t: &Vec<Vec<usize>>) -> bool {
    if now == t.len() - 1 {
        perm.push(now);
        return true;
    }

    for &to in &t[now] {
        if to == par {
            continue;
        }
        if dfs(to, now, perm, t) {
            perm.push(now);
            return true;
        }
    }

    false
}

fn dfs2(now: usize, par: usize, k: usize, perm: &mut Vec<usize>, t: &Vec<Vec<usize>>) -> usize {
    let mut res = 1;
    for &to in &t[now] {
        if to == par {
            continue;
        }
        if to == 0 || !perm.is_empty() && *perm.last().unwrap() == to {
            if perm.len() == k {
                continue;
            }

            perm.pop().unwrap();
        }

        res += dfs2(to, now, k, perm, t);
    }

    res
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut perm = vec![];
    dfs(0, 0, &mut perm, &t);
    perm.reverse();
    // eprintln!("perm: {:?}", perm);
    perm.pop().unwrap();
    perm.remove(0);

    let k = (perm.len() + 1) / 2;
    let white = dfs2(n - 1, n - 1, k, &mut perm, &t);
    let black = n - white;

    // eprintln!("white: {:?}", white);
    // eprintln!("perm: {:?}", perm);

    if black > white {
        println!("Fennec")
    } else {
        println!("Snuke")
    }
}
