#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, a: [usize; n], p: [(usize, usize); m]};

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut costs = vec![0; n];
    for now in 0..n {
        for to in &t[now] {
            costs[now] += a[*to];
        }
    }

    let mut nt = costs.iter().enumerate().map(|(i, v)| std::cmp::Reverse((*v, i))).collect::<std::collections::BinaryHeap<std::cmp::Reverse<(usize, usize)>>>();

    let mut ck = std::collections::HashSet::new();
    let mut res = 0;
    while let Some(std::cmp::Reverse((cost, now))) = nt.pop() {
        if ck.contains(&now) {
            continue;
        }
        ck.insert(now);
        res = std::cmp::max(res, cost);
        costs[now] = 0;

        for to in &t[now] {
            if costs[*to] > 0 {
                costs[*to] -= a[now];
                nt.push(std::cmp::Reverse((costs[*to], *to)));
            }
        }
    }

    println!("{}", res);
}
