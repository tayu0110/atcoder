use itertools::Itertools;
use proconio::*;

fn main() {
    input! {k: usize}

    let mut res = vec![];
    let d = k % 50;
    for _ in 0..50 - d {
        res.push(49 - d);
    }
    res.resize(d, 50);

    let a = k / 50;
    for i in 0..50 {
        res[i] += a;
    }

    println!("{}", res.len());
    println!("{}", res.into_iter().join(" "))
}
