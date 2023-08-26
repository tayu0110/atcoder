use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}
    let mut res = vec![];
    for i in 1..=n {
        res.push(p.iter().position(|v| v == &i).unwrap() + 1);
    }
    println!("{}", res.iter().join(" "))
}
