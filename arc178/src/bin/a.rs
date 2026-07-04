use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [u32; m]}

    let mut memo = vec![false; n];
    for a in a {
        memo[a as usize - 1] = true;
    }

    if memo[0] || memo[n - 1] {
        println!("-1");
        return;
    }
    let mut res = (1..=n).collect::<Vec<_>>();
    for i in 1..n - 1 {
        if memo[i] {
            res.swap(i, i + 1);
        }
    }

    println!("{}", res.iter().join(" "))
}
