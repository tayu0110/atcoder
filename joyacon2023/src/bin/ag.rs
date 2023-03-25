use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, s: [i64; n]}
    let mut res = vec![s[0]];
    for i in 0..n - 1 {
        res.push(s[i + 1] - s[i]);
    }

    println!("{}", res.iter().join(" "))
}
