use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64, i64); m]}

    let mut t = vec![vec![]; n];
    for (a, b, x, y) in p {
        t[a - 1].push((b - 1, x, y));
        t[b - 1].push((a - 1, -x, -y));
    }

    let mut res = vec![None; n];
    res[0] = Some((0, 0));
    let mut nt = VecDeque::new();
    nt.push_back(0);
    while let Some(now) = nt.pop_front() {
        let (px, py) = res[now].unwrap();
        for &(to, x, y) in &t[now] {
            if let Some((nx, ny)) = res[to] {
                if nx != x + px || ny != y + py {
                    res[to] = Some((i64::MAX, i64::MAX));
                    return;
                }
            } else {
                res[to] = Some((x + px, y + py));
                nt.push_back(to);
            }
        }
    }

    for v in res {
        if let Some((x, y)) = v {
            if x == i64::MAX {
                println!("undecidable");
            } else {
                println!("{x} {y}")
            }
        } else {
            println!("undecidable");
        }
    }
}
