use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, query: [(usize, i64); q]}

    let mut row_major = BTreeSet::new();
    let mut col_major = BTreeSet::new();
    // top, left, bottom, right
    row_major.insert((2, -2, n as i64 - 1, 1 - n as i64));
    // left, top, right, bottom
    col_major.insert((2, -2, n as i64 - 1, 1 - n as i64));

    for (t, x) in query {
        if t == 1 {
            if let Some(&(left, top, right, bottom)) = col_major.range(..=(x, 0, 0, 0)).next_back()
            {
                if x < left || right < x || -top > 2 {
                    continue;
                }
                row_major.remove(&(-top, -left, -bottom, -right));
                col_major.remove(&(left, top, right, bottom));

                if left < x {
                    col_major.insert((left, top, x - 1, bottom));
                    row_major.insert((-top, -left, -bottom, -(x - 1)));
                }

                if x < right {
                    col_major.insert((x + 1, top, right, bottom));
                    row_major.insert((-top, -(x + 1), -bottom, -right));
                }
            }
        } else if let Some(&(top, left, bottom, right)) =
            row_major.range(..=(x, std::i64::MAX, 0, 0)).next_back()
        {
            if x < top || bottom < x || -left > 2 {
                continue;
            }
            row_major.remove(&(top, left, bottom, right));
            col_major.remove(&(-left, -top, -right, -bottom));

            if top < x {
                row_major.insert((top, left, x - 1, right));
                col_major.insert((-left, -top, -right, -(x - 1)));
            }

            if x < bottom {
                row_major.insert((x + 1, left, bottom, right));
                col_major.insert((-left, -(x + 1), -right, -bottom));
            }
        }
    }

    println!(
        "{}",
        row_major
            .into_iter()
            .map(|(a, b, c, d)| (c - a + 1) * (b - d + 1))
            .sum::<i64>()
    )
}
