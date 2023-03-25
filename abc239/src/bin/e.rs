#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn rec(now: usize, par: usize, x: &Vec<usize>, memo: &mut Vec<Vec<usize>>, t: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut v = vec![vec![x[now]]];
    for to in &t[now] {
        if *to == par {
            continue;
        }
        v.push(rec(*to, now, x, memo, t));
    }

    let mut v = v.into_iter().flatten().collect::<Vec<_>>();
    v.sort_by_key(|c| std::cmp::Reverse(*c));
    memo[now] = v.into_iter().take(20).collect();
    memo[now].clone()
}

#[fastout]
fn main() {
    input! {n: usize, q: usize, x: [usize; n], p: [(usize, usize); n-1], q: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut memo = vec![vec![]; n];
    rec(0, std::usize::MAX, &x, &mut memo, &t);

    for (v, k) in q {
        println!("{}", memo[v-1][k-1]);
    }
}
