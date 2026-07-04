use itertools::Itertools;
use mincost_flow::MinCostFlow;
use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize, i64); n]}

    // 0: source, 1-150: Ai, 151-300: Bi, 301: dest
    let mut mcf = MinCostFlow::new(302);
    for &(a, b, c) in &e {
        mcf.add_edge(a, b + 150, 1, -c);
    }
    for i in 1..=150 {
        mcf.add_edge(0, i, 1, 0);
        mcf.add_edge(i + 150, 301, 1, 0);
    }
    let mut sum = 0;
    let mut res = vec![];
    for _ in 1..=150 {
        let (flow, cost) = mcf.flow(0, 301, 1);
        // eprintln!("k: {k}, flow: {flow}, cost: {cost}");
        if flow != 0 {
            break;
        }
        sum -= cost;
        res.push(sum);
    }
    println!("{}", res.len());
    println!("{}", res.iter().join("\n"))
}
