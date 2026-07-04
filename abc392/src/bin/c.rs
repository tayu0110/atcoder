use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]}

    let mut res = vec![0; n + 1];
    for (i, &nq) in q.iter().enumerate() {
        res[nq] = q[p[i] - 1];
    }

    println!("{}", res[1..].iter().join(" "))
}
