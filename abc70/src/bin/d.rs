use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n-1], q: usize, k: usize, q: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut dist = vec![std::usize::MAX; n];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, k - 1)));
    while let Some(std::cmp::Reverse((nd, now))) = nt.pop() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;

        for &(to, weight) in &t[now] {
            if dist[to] == std::usize::MAX {
                nt.push(std::cmp::Reverse((nd + weight, to)));
            }
        }
    }

    for (x, y) in q {
        println!("{}", dist[x - 1] + dist[y - 1])
    }
}
