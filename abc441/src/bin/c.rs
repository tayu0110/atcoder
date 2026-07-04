use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, x: usize, mut a: [usize; n]}

    a.sort_unstable_by_key(|&a| Reverse(a));
    let a = a.iter().copied().rev().take(k).rev().collect::<Vec<_>>();
    if a.iter().sum::<usize>() < x {
        println!("-1");
        return;
    }

    let mut sum = 0;
    for (i, a) in a.into_iter().enumerate() {
        sum += a;
        if sum >= x {
            println!("{}", i + n - k + 1);
            break;
        }
    }
}
