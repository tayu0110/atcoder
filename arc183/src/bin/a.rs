use std::iter::repeat;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    if n == 1 {
        println!("{}", repeat(n).take(k).join(" "))
    } else if n % 2 == 0 {
        print!("{}", n / 2);
        println!(
            " {}",
            (1..=n)
                .rev()
                .map(|i| (0..k - (i == n / 2) as usize).map(move |_| i))
                .flatten()
                .join(" ")
        )
    } else {
        print!("{}", repeat((n + 1) / 2).take(k).join(" "));
        print!(" {}", n / 2);
        println!(
            " {}",
            (1..=n)
                .filter(|&i| i != (n + 1) / 2)
                .rev()
                .map(|i| (0..k - (i == n / 2) as usize).map(move |_| i))
                .flatten()
                .join(" ")
        )
    }
}
