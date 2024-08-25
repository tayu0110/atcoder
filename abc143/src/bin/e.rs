#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};
const INF: usize = 0x3f3f3f3f3f3f3f3f;

fn dijkstra(start: usize, l: usize, t: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let n = t.len();
    let mut dist = vec![(INF, INF); n];
    let mut ck = vec![false; n];

    dist[start] = (0, INF - l);
    let mut now = start;
    for _ in 0..n {
        ck[now] = true;
        let (d, m) = dist[now];
        let m = INF - m;
        for (to, c) in &t[now] {
            if *c > l {
                continue;
            }
            let mut nd = d;
            let nm = if m < *c {
                nd += 1;
                INF - (l - *c)
            } else {
                INF - (m - *c)
            };
            if (nd, nm) < dist[*to] {
                dist[*to] = (nd, nm);
            }
        }

        let mut min = (INF, INF);
        for j in 0..n {
            if !ck[j] && dist[j] < min {
                now = j;
                min = dist[j];
            }
        }
    }

    dist.into_iter().unzip::<usize, usize, Vec<usize>, Vec<usize>>().0
}

fn main() {
    input! {n: usize, m: usize, l: usize, p: [(usize, usize, usize); m], q: usize, q: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a-1].push((b-1, c));
        t[b-1].push((a-1, c));
    }

    let mut dist = vec![];
    for i in 0..n {
        dist.push(dijkstra(i, l, &t));
    }

    for (s, t) in q {
        if dist[s-1][t-1] == INF {
            println!("-1");
        } else {
            println!("{}", dist[s-1][t-1]);
        }
    }
}
