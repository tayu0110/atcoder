use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}

    let mut res = vec![];
    for _ in 0..n {
        for j in 0..n - 1 {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                a[j + 1] += k;
                res.push(j + 1);
            }
        }
    }

    eprintln!("{a:?}");
    println!("{}", res.len());
    println!("{}", res.iter().join("\n"))
}
