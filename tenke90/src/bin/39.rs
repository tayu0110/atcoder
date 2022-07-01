use std::collections::VecDeque;

use proconio::input;

const INF: usize = 111222333444;

fn dfs(now: usize, par: usize, depth: usize, rem: &mut usize, t: &Vec<Vec<usize>>) -> usize {
    let mut children = vec![];

    for to in &t[now] {
        if *to != par {
            children.push(dfs(*to, now, depth+1, rem, t));
        }
    }

    let res = children.iter().fold(0, |sum, x| sum + *x);
    let mut m = res;
    let mut s = 0;
    for child in children {
        m -= child;
        s += child * m;
    }

    *rem += s * depth * 2;

    if res != 0 {
        *rem += res * depth * 2;
    }

    res+1
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]};

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut dist = vec![INF; n];
    let mut nt = VecDeque::new();
    nt.push_back((0, 0));
    while !nt.is_empty() {
        let (now, d) = nt.pop_front().unwrap();
        if dist[now] != INF {
            continue;
        }
        dist[now] = d;
        for to in &t[now] {
            if dist[*to] == INF {
                nt.push_back((*to, d+1));
            }
        }
    }

    // eprintln!("{:?}", dist);

    let res = dist.iter().fold(0, |sum, x| sum + *x * (n-1));
    let mut rem = 0;

    dfs(0, INF, 0, &mut rem, &t);

    println!("{}", res - rem);
}