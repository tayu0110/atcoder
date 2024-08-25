use flow::Dinic;
use proconio::*;

fn main() {
    input! {p: usize, q: usize, s: [marker::Chars; p], e: [(i64, i64); p], f: [(i64, i64); q]}

    let mut res = 0;
    let mut ff = Dinic::new(p + q + 2);
    for (i, &(a, b)) in e.iter().enumerate() {
        for (j, &(c, d)) in f.iter().enumerate() {
            if s[i][j] == '1' && a + c - b - d > 0 {
                ff.set_edge(i, j + p, a + c - b - d);
            }
        }
    }

    for (i, (_, e)) in e.into_iter().enumerate() {
        res += e;
        ff.set_edge(p + q, i, i64::MAX);
    }
    for (i, (_, f)) in f.into_iter().enumerate() {
        res += f;
        ff.set_edge(i + p, p + q + 1, i64::MAX);
    }

    println!("{}", ff.flow(p + q, p + q + 1) + res);
}
