#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![std::collections::BinaryHeap::new(); n];
    for (a, b) in p {
        t[a-1].push(std::cmp::Reverse(b-1));
        t[b-1].push(std::cmp::Reverse(a-1));
    }

    let mut stack = vec![0];
    let mut reached = vec![false; n];
    let mut res = vec![];
    while let Some(now) = stack.pop() {
        reached[now] = true;
        res.push(now+1);

        while let Some(std::cmp::Reverse(to)) = t[now].pop() {
            if reached[to] {
                continue;
            }

            stack.push(now);
            stack.push(to);
            break;
        }
    }

    println!("{}", res.iter().join(" "));
}
