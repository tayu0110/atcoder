use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = vec!['-'; n];
    t[n / 2] = '=';
    if n % 2 == 0 {
        t[n / 2 - 1] = '=';
    }

    println!("{}", t.iter().join(""))
}
