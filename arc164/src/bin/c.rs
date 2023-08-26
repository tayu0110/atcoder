use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut nt = p
        .into_iter()
        .map(|(a, b)| (a - b, a, b))
        .collect::<BinaryHeap<_>>();
    let mut res = 0;
    for _ in 0..n {
        let (_, a, b) = nt.pop().unwrap();
        nt.push((b - a, b, a));

        let (_, a, _) = nt.pop().unwrap();
        res += a;
    }

    println!("{}", res)
}
