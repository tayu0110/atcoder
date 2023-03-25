#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in p.into_iter().map(|(u, v)| (u - 1, v - 1)) {
        t[u].push(v);
        t[v].push(u);
    }

    // 0: undfined, 1: black, 2: white
    let mut state = vec![0; n];
    let mut nt = std::collections::VecDeque::new();
    let mut groups = vec![];
    let mut group_number = vec![0; n];
    for i in 0..n {
        if state[i] > 0 {
            continue;
        }

        let (mut black, mut white) = (1, 0);
        state[i] = 1;
        group_number[i] = groups.len();
        nt.push_back(i);
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if state[to] == 0 {
                    state[to] = 3 - state[now];
                    group_number[to] = groups.len();
                    if state[to] == 1 {
                        black += 1;
                    } else {
                        white += 1;
                    }
                    nt.push_back(to);
                } else {
                    if state[to] != 3 - state[now] {
                        println!("0");
                        return;
                    }
                }
            }
        }

        groups.push((black, white));
    }

    let mut res = 0usize;
    for i in 0..n {
        let (b, w) = groups[group_number[i]];
        res += n - (b + w);
        if state[i] == 1 {
            res += w - t[i].len();
        } else {
            res += b - t[i].len();
        }
    }

    println!("{}", res / 2);
}
