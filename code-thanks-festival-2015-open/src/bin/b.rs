use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a1: usize, a2: usize, b1: usize, b2: usize, c: usize}

    let mut res: Vec<usize> = vec![];
    let a = [a1, a2];
    let b = [b1, b2];
    if a.contains(&c) && b.contains(&c) {
        res.extend(a.iter());
        res.extend(b.iter());
    } else if a.contains(&c) {
        res.extend(b.iter());
    } else if b.contains(&c) {
        res.extend(a.iter());
    }

    res.sort();
    res.dedup();
    println!("{}", res.len());
    println!("{}", res.iter().join("\n"))
}
