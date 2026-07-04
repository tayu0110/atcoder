use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize, char); m]}

    for _ in 0..2 {
        let mut row = BTreeMap::new();
        for &(x, y, c) in &p {
            if c == 'W' {
                let entry = row.entry(x).or_insert((0, y));
                entry.1 = entry.1.min(y);
            } else {
                let entry = row.entry(x).or_insert((y, n + 1));
                entry.0 = entry.0.max(y);
            }
        }

        let mut max = 0;
        for &(black, white) in row.values().rev() {
            if white <= max {
                println!("No");
                return;
            }
            max = max.max(black);
        }

        for i in 0..m {
            p[i] = (p[i].1, p[i].0, p[i].2);
        }
    }

    println!("Yes")
}
