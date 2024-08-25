use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, q: [(usize, usize, usize); q]}

    let mut a = vec![0; n];
    let mut written = vec![false; n];
    for (l, r, t) in q.into_iter().rev() {
        for i in l - 1..r {
            if !written[i] {
                written[i] = true;
                a[i] = t;
            }
        }
    }

    println!("{}", a.iter().join("\n"))
}
