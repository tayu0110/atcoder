#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize, k: usize, p: [(usize, usize); m], a: [usize; n], b: [usize; k]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut dist = vec![std::usize::MAX; n];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 0)));

    while let Some(std::cmp::Reverse((mut nd, now))) = nt.pop() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        
        if nd < k && a[now] == b[nd] {
            nd += 1;
        }
        dist[now] = nd;

        for to in &t[now] {
            if dist[*to] == std::usize::MAX {
                nt.push(std::cmp::Reverse((nd, *to)));
            }
        }
    }

    if dist[n-1] == k {
        println!("Yes");
    } else {
        println!("No");
    }
}
