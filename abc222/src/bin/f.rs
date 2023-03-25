#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn rec(now: usize, par: usize, d: &Vec<i64>, memo: &mut Vec<Vec<i64>>, t: &Vec<Vec<(usize, i64)>>) -> i64 {
    let mut res = 0;
    for (to, c) in &t[now] {
        if *to == par {
            memo[now].push(-1);
            continue;
        }

        let r = rec(*to, now, d, memo, t);
        memo[now].push(r + *c);
        res = std::cmp::max(res, r + *c);
    }

    std::cmp::max(res, d[now])
}

fn rec2(now: usize, score: i64, d: &Vec<i64>, memo: &mut Vec<Vec<i64>>, t: &Vec<Vec<(usize, i64)>>) {
    let mut par = std::usize::MAX;
    let mut max = (0, 0);
    let mut second = (0, 0);
    for (i, &(_, c)) in t[now].iter().enumerate() {
        if memo[now][i] < 0 {
            memo[now][i] = score + c;
            par = i;
        }
        if memo[now][i] > max.0 {
            second = max;
            max = (memo[now][i], i);
        } else if memo[now][i] > second.0 {
            second = (memo[now][i], i);
        }
    }

    for (i, &(to, _)) in t[now].iter().enumerate() {
        if i == par {
            continue;
        }
        if max.1 == i {
            rec2(to, std::cmp::max(second.0, d[now]), d, memo, t);
        } else {
            rec2(to, std::cmp::max(max.0, d[now]), d, memo, t);
        }
    }
}

#[fastout]
fn main() {
    input! {n: usize, p: [(usize, usize, i64); n-1], d: [i64; n]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a-1].push((b-1, c));
        t[b-1].push((a-1, c));
    }

    let mut memo = vec![vec![]; n];
    rec(0, std::usize::MAX, &d, &mut memo, &t);

    rec2(0, 0, &d, &mut memo, &t);

    for i in 0..n {
        println!("{}", memo[i].iter().max().unwrap());
    }
}
