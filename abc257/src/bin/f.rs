use std::{collections::VecDeque};

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, m: usize, p: [(usize, usize); m]};

    let mut t = vec![vec![]; n+1];
    for (u, v) in p {
        t[u].push(v);
        t[v].push(u);
    }

    const INF: usize = 111222333444555666;
    let mut d = vec![vec![INF; n+1]; 3];
    for i in 0..3 {
        let target = if i < 2 { i } else { n };
        let mut nt = VecDeque::new();
        nt.push_back((target, 0));
        while !nt.is_empty() {
            let (now, nd) = nt.pop_front().unwrap();
            if d[i][now] != INF {
                continue;
            }
            d[i][now] = nd;
            for to in &t[now] {
                if d[i][*to] != INF {
                    continue;
                }
                nt.push_back((*to, nd+1));
            }
        }
    }

    for i in 1..n+1 {
        let tmp = d[0][i];
        d[0][i] = 0;
        let res = std::cmp::min(d[1][n], std::cmp::min(d[1][0], d[1][i]) + std::cmp::min(d[2][i], d[0][n]));
        if res == INF {
            println!("-1");
        } else {
            println!("{}", res);
        }
        d[0][i] = tmp;
    }
}
