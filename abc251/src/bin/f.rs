use std::collections::VecDeque;

use proconio::*;
use unionfind::UnionFind;

fn dfs_tree(t: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = t.len();
    let mut stack = vec![(0, 0)];
    let mut uf = UnionFind::new(n);
    let mut res = vec![vec![]; n];
    while let Some((now, next)) = stack.pop() {
        for i in next..t[now].len() {
            let to = t[now][i];
            if uf.is_same(now, to) {
                continue;
            }

            res[now].push(to);
            stack.push((now, i + 1));
            stack.push((to, 0));
            uf.merge(now, to);
            break;
        }
    }

    res
}

fn bfs_tree(t: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = t.len();
    let mut nt = VecDeque::new();
    nt.push_back(0);
    let mut uf = UnionFind::new(n);
    let mut res = vec![vec![]; n];
    while let Some(now) = nt.pop_back() {
        for &to in &t[now] {
            if uf.is_same(now, to) {
                continue;
            }

            res[now].push(to);
            nt.push_back(to);
            uf.merge(now, to);
        }
    }

    res
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let s = dfs_tree(&t);
    let t = bfs_tree(&t);
    let mut cnt = 0;
    for (i, v) in s.iter().enumerate() {
        for v in v {
            println!("{} {}", i + 1, v + 1);
            cnt += 1;
        }
    }
    assert_eq!(cnt, n - 1);
    let mut cnt = 0;
    for (i, v) in t.iter().enumerate() {
        for v in v {
            println!("{} {}", i + 1, v + 1);
            cnt += 1;
        }
    }
    assert_eq!(cnt, n - 1);
}
