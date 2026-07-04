use std::collections::VecDeque;

use proconio::*;

fn solve(d: i64, mut x: Vec<i64>) -> Vec<i64> {
    x.sort_unstable();

    let n = x.len() as i64;
    let mut x = VecDeque::from(x);

    let mut sum = x.iter().sum::<i64>();
    let mut presum = 0;
    let mut now = x[0] - d;
    let &last = x.back().unwrap();
    let mut res = vec![];
    while now <= last + d {
        while let Some(pop) = x.front().filter(|&&f| f < now) {
            presum += pop;
            sum -= pop;
            x.pop_front();
        }

        res.push(sum - now * x.len() as i64 + now * (n - x.len() as i64) - presum);
        now += 1;
    }
    res
}

fn main() {
    input! {n: usize, d: i64, p: [(i64, i64); n]}

    let (x, y) = p.into_iter().unzip::<i64, i64, Vec<_>, Vec<_>>();
    let x = solve(d, x);
    let mut y = solve(d, y);

    y.sort_unstable();

    let mut res = 0;
    for x in x {
        res += y.partition_point(|y| x + y <= d);
    }

    println!("{res}")
}
