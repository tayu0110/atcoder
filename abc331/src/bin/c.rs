use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut p = a.into_iter().enumerate().collect::<Vec<_>>();
    p.sort_unstable_by_key(|v| v.1);

    let mut sum = 0;
    let mut prev = usize::MAX;
    let mut pending = 0;
    let mut res = vec![0; n];
    for (i, p) in p.into_iter().rev() {
        if prev > p {
            prev = p;
            sum += pending;
            pending = 0;
        }
        pending += p;

        res[i] = sum;
    }

    println!("{}", res.iter().join(" "))
}
