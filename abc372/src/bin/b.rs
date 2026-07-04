use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut m: usize}

    let mut res = vec![];
    for i in (0..30).rev() {
        while m >= 3usize.pow(i) {
            res.push(i);
            m -= 3usize.pow(i);
        }
    }

    for _ in 0..m {
        res.push(0);
    }

    println!("{}", res.len());
    println!("{}", res.iter().join(" "))
}
