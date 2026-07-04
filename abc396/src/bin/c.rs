use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut b: [i64; n], mut w: [i64; m]}

    b.sort_unstable_by_key(|&b| Reverse(b));
    w.sort_unstable_by_key(|&w| Reverse(w));

    let mut res = 0;
    let mut sum = 0;
    let mut cursor = 0;
    for (i, b) in b.into_iter().enumerate() {
        sum += b;
        while cursor <= i.min(m - 1) && w[cursor] > 0 {
            sum += w[cursor];
            cursor += 1;
        }

        res = res.max(sum);
    }

    println!("{res}")
}
