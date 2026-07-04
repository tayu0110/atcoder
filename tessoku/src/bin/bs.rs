use mincost_flow::MinCostFlow;
use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n], b: [i64; n]}

    // 0..n     : Homework
    // n..n*2   : Temperature
    // n*2      : source
    // n*2+1    : dest
    let mut mcf = MinCostFlow::new(n + n + 2);
    for i in 0..n {
        mcf.add_edge(n * 2, i, 1, 0);
        mcf.add_edge(n + i, n * 2 + 1, 1, 0);
        for j in 0..n {
            mcf.add_edge(i, n + j, 1, a[i] * b[j]);
        }
    }

    let (_, cost) = mcf.flow(n * 2, n * 2 + 1, n as i64);
    println!("{cost}");
}
