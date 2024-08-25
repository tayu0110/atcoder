use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut n: usize}

    n -= 1;
    let mut res = vec![];
    while n > 0 {
        res.push(n % 5);
        n /= 5;
    }

    if res.is_empty() {
        res.push(0);
    }

    res.reverse();
    println!("{}", res.iter().map(|&r| r * 2).join(""))
}
