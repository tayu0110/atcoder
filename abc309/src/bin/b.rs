use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [marker::Chars; n]}

    let mut t = a.clone();
    for i in 0..n - 1 {
        t[0][i + 1] = a[0][i];
        t[i + 1][n - 1] = a[i][n - 1];
    }
    for i in 1..n {
        t[n - 1][i - 1] = a[n - 1][i];
        t[i - 1][0] = a[i][0];
    }

    for t in t {
        println!("{}", t.iter().join(""))
    }
}
