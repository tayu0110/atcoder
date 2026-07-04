use std::fmt::Write as _;

use proconio::*;

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {n: usize, e: [(usize, usize); n]}

        let mut p = e.iter().map(|&(w, p)| (w + p, w, p)).collect::<Vec<_>>();
        p.sort_unstable();

        let mut sum = e.iter().map(|v| v.0).sum::<usize>();
        while let Some((_, w, p)) = p.pop() {
            if sum <= w + p {
                break;
            }
            sum -= w + p;
        }

        // eprintln!("sum: {sum}, p: {p:?}");
        writeln!(buf, "{}", p.len()).unwrap();
    }

    print!("{buf}")
}
