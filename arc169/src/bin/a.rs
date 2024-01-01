use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n], p: [usize; n - 1]}

    let mut t = vec![vec![]; n];
    for (i, p) in p.into_iter().enumerate() {
        t[p - 1].push(i + 1);
    }

    let mut nt = VecDeque::new();
    nt.push_back(0);
    let mut dist = vec![usize::MAX; n];
    dist[0] = 0;
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            dist[to] = dist[now] + 1;
            nt.push_back(to);
        }
    }

    let &max = dist.iter().max().unwrap();

    let mut res = vec![0; max + 1];
    for i in 0..n {
        res[dist[i]] += a[i];
    }

    for i in (0..=max).rev() {
        if res[i] != 0 {
            if res[i] < 0 {
                println!("-")
            } else {
                println!("+")
            }
            return;
        }
    }

    println!("0")
}
