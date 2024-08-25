#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn solve(n: usize, c: &[i64]) -> Vec<i64> {
    let mut t = vec![0; n];

    for (i, v) in c.iter().enumerate() {
        t[i+1] = t[i] + *v;
    }

    let mut res = vec![0];
    let mut max = t.last().unwrap();
    for v in t.iter().rev().skip(1) {
        let r = std::cmp::max(*max - *v, 0);
        res.push(r);
        max = std::cmp::max(max, v);
    }

    res.reverse();
    res
}

fn main() {
    input! {n: usize, c: [i64; n-1]};

    let res1 = solve(n, &c);

    let mut c = c;
    c.reverse();
    let mut res2 = solve(n, &c);
    res2.reverse();

    for (u, v) in res1.iter().zip(res2.iter()) {
        println!("{}", std::cmp::max(u, v));
    }
}
