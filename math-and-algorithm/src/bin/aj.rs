use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, p: [(usize, usize, i64); q]}

    let mut t = vec![0; n + 1];
    for (l, r, x) in p {
        t[l - 1] += x;
        t[r] -= x;
    }
    for i in 0..n {
        t[i + 1] += t[i];
    }

    println!(
        "{}",
        t[..n]
            .windows(2)
            .map(|v| match v[0].cmp(&v[1]) {
                std::cmp::Ordering::Equal => "=",
                std::cmp::Ordering::Less => "<",
                std::cmp::Ordering::Greater => ">",
            })
            .join("")
    )
}
