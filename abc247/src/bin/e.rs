#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn solve(x: i64, y: i64, a: &[i64]) -> i64 {
    let mut px = vec![];
    let mut py = vec![];
    for (i, a) in a.iter().enumerate() {
        if *a == x {
            px.push(i as i64);
        }
        if *a == y {
            py.push(i as i64);
        }
    }
    if px.is_empty() || py.is_empty() {
        return 0;
    }
    px.push(a.len() as i64);
    py.push(a.len() as i64);
    px.dedup();
    py.dedup();

    let mut res = a.len() as i64 * (a.len() as i64 + 1) / 2;
    let mut v = vec![];
    {
        let mut prev = 0i64;
        for next in px {
            v.push(next);
            if next > 0 {
                let range = next - prev;
                res -= range * (range + 1) / 2;
            }
            prev = next + 1;
        }
    }
    {
        let mut prev = 0;
        for next in py {
            v.push(next);
            if next > 0 {
                let range = next - prev;
                res -= range * (range + 1) / 2;
            }
            prev = next + 1;
        }
    }
    v.pop();

    v.sort();
    v.dedup();
    let mut prev = 0;
    for next in v {
        if next > 0 {
            let range = next - prev;
            res += range * (range + 1) / 2;
        }
        prev = next + 1;
    }

    res
}

#[fastout]
fn main() {
    input! {n: usize, x: i64, y: i64, a: [i64; n]}

    let mut res = 0;
    let mut v = vec![];
    for a in a {
        if a < y || x < a {
            res += solve(x, y, &v);
            v = vec![];
        } else {
            v.push(a);
        }
    }

    if !v.is_empty() {
        res += solve(x, y, &v);
    }

    println!("{}", res);
}
