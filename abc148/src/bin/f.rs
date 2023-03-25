#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

const INF: usize = 0x3f3f3f3f3f3f3f3f;

fn bfs(start: usize, t: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut nt = std::collections::VecDeque::new();
    let mut dist = vec![INF; t.len()];
    nt.push_back(start);
    dist[start] = 0;
    while let Some(now) = nt.pop_front() {
        for to in &t[now] {
            if dist[*to] == INF {
                dist[*to] = dist[now] + 1;
                nt.push_back(*to);
            }
        }
    }

    dist
}

fn main() {
    input! {n: usize, u: usize, v: usize, p: [(usize, usize); n-1]};
    let (u, v) = (u-1, v-1);

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let aoki = bfs(v, &t);
    let mut dist = vec![INF; n];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back(u);
    dist[u] = 0;

    while let Some(now) = nt.pop_front() {
        for to in &t[now] {
            if dist[*to] == INF {
                if dist[now] + 1 >= aoki[*to] {
                    continue;
                }
                dist[*to] = dist[now] + 1;
                nt.push_back(*to);
            }
        }
    }

    let mut res = 0;
    for (i, v) in dist.iter().enumerate() {
        if *v != INF {
            res = std::cmp::max(res, aoki[i]-1);
        }
    }

    println!("{}", res);
}
