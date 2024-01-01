use std::io::Write;

use proconio::*;

fn solve(l: usize, r: usize) -> usize {
    let range = r - l;
    if range == 1 << range.trailing_zeros() {
        println!("? {l} {r}");
        std::io::stdout().flush().ok();
        input_interactive!(res: usize);
        return res;
    }

    let mut np = range.next_power_of_two();
    if np > range {
        np >>= 1;
    }

    let s = range - np;
    let t = 2 * np - range;
    if s.count_ones() < t.count_ones() + 1 {
        println!("? {} {}", l, l + np);
        std::io::stdout().flush().ok();
        input_interactive!(res: usize);

        res + solve(l + np, r)
    } else {
        println!("? {} {}", l, l + np);
        std::io::stdout().flush().ok();
        input_interactive!(res0: usize);
        println!("? {} {}", r - np, r);
        std::io::stdout().flush().ok();
        input_interactive!(res1: usize);

        res0 + res1 - solve(r - np, l + np)
    }
}

fn main() {
    input_interactive!(n: usize);

    let (l, r) = (0, n);
    println!("! {}", solve(l, r));
}
