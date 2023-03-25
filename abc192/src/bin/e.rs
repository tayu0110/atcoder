#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, x: usize, y: usize, p: [(usize, usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b, nt, k) in p {
        t[a-1].push((b-1, nt, k));
        t[b-1].push((a-1, nt, k));
    }

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, x-1)));
    let mut dist = vec![std::usize::MAX; n];
    while let Some(std::cmp::Reverse((nd, now))) = nt.pop() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;

        for (to, d, k) in &t[now] {
            if dist[*to] == std::usize::MAX {
                if nd % k == 0 {
                    nt.push(std::cmp::Reverse((nd + *d, *to)));
                } else {
                    nt.push(std::cmp::Reverse(((nd / k + 1) * k + *d, *to)));
                }
            }
        }
    }

    if dist[y-1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[y-1]);
    }
}
