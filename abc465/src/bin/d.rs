use itertools::Itertools;
use proconio::*;

fn solve(x: usize, y: usize, k: usize) -> usize {
    if x == y {
        return 0;
    }

    if x > y {
        solve(x / k, y, k) + 1
    } else {
        solve(x, y / k, k) + 1
    }
}

fn main() {
    input! {t: usize}

    let mut ret = vec![];
    for _ in 0..t {
        input! {x: usize, y: usize, k: usize}

        ret.push(solve(x, y, k));
    }
    println!("{}", ret.iter().join("\n"))
}
