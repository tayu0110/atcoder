use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (x, y, z) in p {
        t[x - 1].push((y - 1, z));
        t[y - 1].push((x - 1, z));
    }

    let mut res = vec![usize::MAX; n];
    let mut nt = VecDeque::new();
    for i in 0..n {
        if res[i] == usize::MAX {
            nt.push_back(i);
            res[i] = 0;
            while let Some(now) = nt.pop_front() {
                for &(to, z) in &t[now] {
                    if res[to] == usize::MAX {
                        res[to] = res[now] ^ z;
                        nt.push_back(to);
                    } else if res[now] ^ z != res[to] {
                        println!("-1");
                        return;
                    }
                }
            }
        }
    }

    println!("{}", res.iter().join(" "))
}
