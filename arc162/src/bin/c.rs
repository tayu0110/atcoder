use proconio::*;
use std::collections::HashSet;

fn dfs(now: usize, k: i32, a: &[i32], t: &Vec<Vec<usize>>) -> (bool, usize, HashSet<i32>) {
    let mut undecided = if a[now] < 0 { 1 } else { 0 };
    let mut mex = HashSet::new();
    if a[now] >= 0 {
        mex.insert(a[now]);
    }
    for &to in &t[now] {
        let (f, ud, set) = dfs(to, k, a, t);
        if f {
            return (true, 0, HashSet::new());
        }

        undecided += ud;

        mex.extend(set.iter());
    }

    if undecided > 1 {
        return (false, undecided, HashSet::new());
    }

    if undecided == 0 {
        if (0..k).all(|v| mex.contains(&v)) && !mex.contains(&k) {
            (true, 0, HashSet::new())
        } else {
            (false, undecided, mex)
        }
    } else {
        if (0..k).filter(|v| !mex.contains(v)).count() <= 1 && !mex.contains(&k) {
            (true, 0, HashSet::new())
        } else {
            (false, undecided, mex)
        }
    }
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: i32, p: [usize; n-1], a: [i32; n]}

        let mut t = vec![vec![]; n];
        for (i, p) in p.into_iter().enumerate() {
            t[p - 1].push(i + 1);
        }

        if dfs(0, k, &a, &t).0 {
            println!("Alice")
        } else {
            println!("Bob")
        }
    }
}
