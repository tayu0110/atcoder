use std::fmt::Write as _;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut a: [usize; n], query: [(usize, usize); q]}
    a.sort_unstable();
    a.dedup();
    a.insert(0, 0);

    let mut buf = String::new();
    for (x, y) in query {
        if *a.last().unwrap() < x {
            writeln!(buf, "{}", x + y - 1).unwrap();
            continue;
        }

        let p = a.partition_point(|&a| a < x);

        let (mut l, mut r) = (x, 5 * 1000_000_000);
        while r - l > 1 {
            let m = (r + l) / 2;
            let pos = a.partition_point(|&a| a < m);
            if m - x - (pos - p) < y {
                l = m;
            } else {
                r = m;
            }
        }
        writeln!(buf, "{l}").unwrap();
    }

    print!("{buf}")
}
