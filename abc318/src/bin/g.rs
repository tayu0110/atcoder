use flow::Dinic;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: usize, b: usize, c: usize, p: [(usize, usize); m]}

    // 0: s, 1..=n: x[1~n], n+1..2*n: y[1~n], 2*n+1: t
    let mut dc = Dinic::new(2 * n + 2);
    dc.set_edge(0, n + b, 2);
    dc.set_edge(n + a, 2 * n + 1, 1);
    dc.set_edge(n + c, 2 * n + 1, 1);
    for i in 1..=n {
        dc.set_edge(i, n + i, 1);
    }
    for (u, v) in p {
        dc.set_edge(n + u, v, 1);
        dc.set_edge(n + v, u, 1);
    }

    let res = dc.flow(0, 2 * n + 1);
    if res == 2 {
        println!("Yes")
    } else if res < 2 {
        println!("No")
    } else {
        unreachable!()
    }
}
