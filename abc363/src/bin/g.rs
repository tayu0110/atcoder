use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut d: [usize; n], mut p: [usize; n]}

    for _ in 0..q {
        input! {c: usize, x: usize, y: usize}

        d[c - 1] = x;
        p[c - 1] = y;
        let mut s = d.iter().cloned().zip(p.iter().cloned()).collect::<Vec<_>>();

        s.sort_unstable_by_key(|(d, p)| (*d, Reverse(*p)));

        let mut now = 1;
        let mut res = 0;
        for (d, p) in s {
            if now <= d {
                res += p;
                now += 1;
            }
        }

        println!("{res}");
    }
}
