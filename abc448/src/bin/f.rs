use cpgraph::manhattan_minimum_spanning_tree;
use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}
    let p = p
        .into_iter()
        .map(|(x, y)| (x - 1, y - 1))
        .collect::<Vec<_>>();

    let ret = manhattan_minimum_spanning_tree(&p);
    let mut t = vec![vec![]; n];
    for (_, from, to) in ret {
        t[from].push(to);
        t[to].push(from);
    }

    let mut check = vec![false; n];
    let mut tour = vec![];
    let mut nt = vec![0];
    while let Some(now) = nt.pop() {
        if check[now] {
            continue;
        }
        check[now] = true;
        tour.push(now);
        for &to in &t[now] {
            if !check[to] {
                nt.push(to);
            }
        }
    }

    assert!(tour.len() == n);
    println!("{}", tour.iter().map(|t| t + 1).join(" "))
}
