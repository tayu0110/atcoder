use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n], b: [usize; n - 1]}

    let mut b = b
        .into_iter()
        .enumerate()
        .map(|(i, b)| (b, i + 1))
        .collect::<BTreeSet<_>>();
    a.sort_unstable();

    let mut res = None;
    while let Some(a) = a.pop() {
        if let Some(&t) = b.range((a, 0)..).next() {
            b.remove(&t);
        } else {
            if res.is_some() {
                println!("-1");
                return;
            } else {
                res = Some(a);
            }
        }
    }

    println!("{}", res.unwrap())
}
