use std::cmp::Reverse;

use proconio::*;

fn make_tree(n: usize, e: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut t = vec![vec![]; n];
    for &(u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }
    t
}

fn diameter(t: &[Vec<usize>]) -> usize {
    let n = t.len();
    let mut index = 0;
    let mut max = 0;
    for _ in 0..2 {
        let mut d = vec![usize::MAX; n];
        let mut stack = vec![index];
        d[index] = 0;
        while let Some(now) = stack.pop() {
            for &to in &t[now] {
                if d[to] == usize::MAX {
                    d[to] = d[now] + 1;
                    stack.push(to);
                }
            }
        }

        for i in 0..n {
            if max < d[i] {
                max = d[i];
                index = i;
            }
        }
    }
    max
}

fn make_table(t: &[Vec<usize>]) -> Vec<usize> {
    let n = t.len();
    let mut memo = vec![vec![]; n];
    for i in 0..n {
        memo[i] = vec![usize::MAX; t[i].len()];
    }

    fn dfs(now: usize, par: usize, t: &[Vec<usize>], memo: &mut [Vec<usize>]) -> usize {
        let mut max = 0;
        for i in 0..t[now].len() {
            let to = t[now][i];
            if to != par {
                let res = dfs(to, now, t, memo) + 1;
                memo[now][i] = res;
                max = max.max(res);
            }
        }
        max
    }
    fn dfs2(now: usize, prop: usize, t: &[Vec<usize>], memo: &mut [Vec<usize>]) {
        let mut buf = vec![];
        let mut skip = usize::MAX;
        for i in 0..t[now].len() {
            if memo[now][i] == usize::MAX {
                memo[now][i] = prop;
                skip = i;
            }
            buf.push(memo[now][i]);
            buf.sort_unstable_by_key(|&b| Reverse(b));
            buf.truncate(3);
        }

        for i in 0..t[now].len() {
            if skip != i {
                let to = t[now][i];
                let prop = if memo[now][i] == buf[0] {
                    *buf.get(1).unwrap_or(&0)
                } else {
                    buf[0]
                };
                dfs2(to, prop + 1, t, memo);
            }
        }
    }

    dfs(0, 0, t, &mut memo);
    dfs2(0, 0, t, &mut memo);

    memo.into_iter()
        .map(|v| *v.iter().max().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    input! {n1: usize, e1: [(usize, usize); n1 - 1], n2: usize, e2: [(usize, usize); n2 - 1]}

    let t1 = make_tree(n1, &e1[..]);
    let t2 = make_tree(n2, &e2[..]);

    let d = diameter(&t1).max(diameter(&t2));
    let tab1 = make_table(&t1);
    let mut tab2 = make_table(&t2);

    tab2.sort_unstable_by_key(|&b| Reverse(b));
    let mut cum = vec![0; n2 + 1];
    for i in 0..n2 {
        cum[i + 1] = cum[i] + tab2[i];
    }
    let mut res = 0usize;
    for t in tab1 {
        let pos = tab2.partition_point(|&s| s + t + 1 > d);
        res += d * (n2 - pos);
        res += cum[pos] + (t + 1) * pos;
    }

    println!("{res}")
}
