use std::collections::HashSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let a = a.into_iter().collect::<HashSet<_>>();
    let mut res = vec![];
    for i in 1..=n {
        if !a.contains(&i) {
            res.push(i);
        }
    }

    println!("{}", res.len());
    println!("{}", res.iter().join(" "))
}
