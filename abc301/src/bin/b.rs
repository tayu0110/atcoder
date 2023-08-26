use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = vec![];
    for v in a.windows(2) {
        let (l, r) = (v[0], v[1]);

        if l < r {
            for i in l..r {
                res.push(i);
            }
        } else {
            for i in (r + 1..=l).rev() {
                res.push(i);
            }
        }
    }

    res.push(a[n - 1]);

    println!("{}", res.iter().join(" "))
}
