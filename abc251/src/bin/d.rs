use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_: usize}

    let mut res = vec![];
    let mut base = 1;
    for _ in 0..3 {
        for i in 1..=100 {
            res.push(i * base);
        }

        base *= 100;
    }

    println!("{}", res.len());
    println!("{}", res.iter().join(" "))
}
