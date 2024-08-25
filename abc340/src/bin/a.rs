use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a: usize, b: usize, d: usize}

    let mut res = vec![];
    let mut now = a;
    while now != b {
        res.push(now);
        now += d;
    }
    res.push(now);

    println!("{}", res.iter().join(" "))
}
