use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashSet;

type HashSet<T> = FxHashSet<T>;

fn dfs(now: usize, stack: &mut Vec<usize>, used: &mut HashSet<usize>, t: &Vec<HashSet<usize>>) {
    stack.push(now);
    used.insert(now);

    for &to in &t[now] {
        if used.contains(&to) {
            continue;
        }

        dfs(to, stack, used, t);
    }
}

fn main() {
    input! {n: usize, q: usize}

    let mut t = vec![HashSet::default(); n];
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {u: usize, v: usize}
            t[u - 1].insert(v - 1);
            t[v - 1].insert(u - 1);
        } else {
            input! {u: usize}

            let mut used = HashSet::default();
            let mut stack = vec![];
            dfs(u - 1, &mut stack, &mut used, &t);
            stack.sort();
            println!("{}", stack.iter().map(|v| v + 1).join(" "))
        }
    }
}
