use proconio::*;
use rustc_hash::FxHashSet;

fn find_root(now: usize, par: usize, res: &mut usize, t: &[Vec<usize>], set: &FxHashSet<usize>) {
    for &to in &t[now] {
        if to != par {
            find_root(to, now, res, t, set);
        }

        if *res != usize::MAX {
            return;
        }
    }

    if set.contains(&now) && *res == usize::MAX {
        *res = now;
    }
}

fn dfs(now: usize, par: usize, t: &[Vec<usize>], set: &FxHashSet<usize>) -> usize {
    let mut res = 0;

    for &to in &t[now] {
        if to != par {
            res += dfs(to, now, t, set);
        }
    }

    if set.contains(&now) || res != 0 {
        res += 1;
    }
    res
}

fn main() {
    input! {n: usize, k: usize, e: [(usize, usize); n - 1], mut v: [usize; k]}
    v.iter_mut().for_each(|v| *v -= 1);

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let set = v.iter().cloned().collect::<FxHashSet<_>>();
    let mut root = usize::MAX;
    find_root(0, 0, &mut root, &t, &set);

    println!("{}", dfs(root, root, &t, &set));
}
