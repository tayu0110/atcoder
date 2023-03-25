use proconio::{input, marker::Chars};
use string::zalgorithm;

fn main() {
    input! {s: Chars, t: Chars}

    let u = s.iter().chain(t.iter()).collect::<String>();

    let z = zalgorithm(u);

    let mut g = vec![vec![]; t.len() + 1];
    for i in 0..t.len() {
        g[i + 1].push((i, 0));
        g[i].push((i + std::cmp::min(z[i + s.len()], s.len()), 1));
    }

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 0)));
    let mut res = vec![std::usize::MAX; t.len() + 1];
    while let Some(std::cmp::Reverse((d, now))) = nt.pop() {
        if res[now] != std::usize::MAX {
            continue;
        }
        res[now] = d;

        for &(to, w) in &g[now] {
            if res[to] == std::usize::MAX {
                nt.push(std::cmp::Reverse((d + w, to)));
            }
        }
    }

    if res[t.len()] == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", res[t.len()])
    }
}
