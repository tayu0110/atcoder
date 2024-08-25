use std::{cmp::Reverse, usize};

use proconio::*;

fn main() {
    input! {n: usize, x: usize, y: usize, a: [usize; n], b: [usize; n]}

    let mut p = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| [a, b])
        .collect::<Vec<_>>();

    let mut res = n;
    for i in 0..2 {
        p.sort_unstable_by_key(|a| Reverse(a[i]));

        let mut sweet = 0;
        let mut solty = 0;
        for (j, &[a, b]) in p.iter().enumerate() {
            sweet += a;
            solty += b;

            if sweet > x || solty > y {
                res = res.min(j + 1);
                break;
            }
        }
    }

    println!("{res}");
}
