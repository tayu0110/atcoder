use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}

    for k in 1..=m {
        for i in 0..n - 1 {
            if a[i] % k > a[i + 1] % k {
                a.swap(i, i + 1);
            }
        }
    }

    println!("{}", a.iter().join("\n"))
}
