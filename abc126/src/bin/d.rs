#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v, w) in p {
        t[u-1].push((v-1, w));
        t[v-1].push((u-1, w));
    }

    const INF: usize = 0x3f3f3f3f3f3f3f3f;
    let mut dist = vec![INF; n];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 0));
    while let Some((now, nd)) = nt.pop_front() {
        if dist[now] != INF {
            continue;
        }
        dist[now] = nd;
        for (to, d) in &t[now] {
            nt.push_back((*to, nd + *d));
        }
    }

    for v in dist {
        println!("{}", v % 2);
    }
}
