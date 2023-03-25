#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64); m]};

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a-1].push((b-1, -c));
    }

    const INF: i64 = 0x3f3f3f3f3f3f3f3f;
    let mut cost = vec![INF; n];
    cost[0] = 0;
    for i in 0..n*2 {
        for now in 0..n {
            for &(to, c) in &t[now] {
                if cost[now] + c < cost[to] {
                    if i < n {
                        cost[to] = cost[now] + c;
                    } else {
                        cost[to] = -INF;
                    }
                }
            }
        }
    }

    if cost[n-1] == -INF {
        println!("inf");
    } else {
        println!("{}", -cost[n-1]);
    }
}
