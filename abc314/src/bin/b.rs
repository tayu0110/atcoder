use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = vec![];
    for _ in 0..n {
        input! {c: usize, a: [usize; c]}

        t.push((c, a));
    }

    input! {x: usize}

    let mut t = t
        .into_iter()
        .enumerate()
        .filter(|v| v.1 .1.contains(&x))
        .map(|v| (v.1 .0, v.0))
        .collect::<Vec<_>>();
    t.sort();

    if t.is_empty() {
        println!("0");
        println!("");
        return;
    }

    let min = t[0].0;
    let mut res = vec![];
    for (c, i) in t {
        if c == min {
            res.push(i + 1);
        }
    }

    res.sort();
    println!("{}", res.len());
    println!("{}", res.iter().join(" "))
}
