use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, n: usize, p: [(usize, usize); n]}

    let set = p.iter().cloned().collect::<HashSet<_>>();

    let mut t = vec![];
    for (a, b) in p {
        for y in a.saturating_sub(2)..=a {
            for x in b.saturating_sub(2)..=b {
                if x < 1 || y < 1 || y + 2 > h || x + 2 > w {
                    continue;
                }
                t.push((y, x));
            }
        }
    }
    t.sort();
    t.dedup();

    let mut res = vec![0; 10];
    for (y, x) in t {
        let mut sum = 0;
        for i in 0..3 {
            for j in 0..3 {
                sum += set.contains(&(y + i, x + j)) as usize;
            }
        }

        res[sum] += 1;
    }

    res[0] = (h - 2) * (w - 2) - res.iter().sum::<usize>();
    for r in res {
        println!("{}", r)
    }
}
