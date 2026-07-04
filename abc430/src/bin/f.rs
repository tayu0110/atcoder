use std::fmt::Write as _;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {n: usize, s: marker::Bytes}

        let mut forward = vec![0; n];
        for (i, &b) in s.iter().enumerate() {
            if b == b'R' {
                forward[i + 1] += forward[i] + 1;
            }
        }
        let mut rforward = vec![0; n];
        for (i, &b) in s.iter().enumerate().rev() {
            if b == b'R' {
                rforward[i] += rforward[i + 1] + 1;
            }
        }
        let mut backward = vec![0; n];
        for (i, &b) in s.iter().enumerate().rev() {
            if b == b'L' {
                backward[i] += backward[i + 1] + 1;
            }
        }
        let mut rbackward = vec![0; n];
        for (i, &b) in s.iter().enumerate() {
            if b == b'L' {
                rbackward[i + 1] += rbackward[i] + 1;
            }
        }

        // eprintln!("forward: {forward:?}, backward: {backward:?}, rforward: {rforward:?}, rbackward: {rbackward:?}");
        let mut cum = vec![0i32; n + 1];
        for i in 0..n {
            if forward[i] + backward[i] < n {
                cum[forward[i] + backward[i]] += 1;
            }
            if rforward[i] + rbackward[i] <= n {
                cum[n - rbackward[i] - rforward[i]] -= 1;
            }
        }
        for i in 0..n {
            cum[i + 1] += cum[i];
        }
        writeln!(buf, "{}", cum.iter().take(n).join(" ")).unwrap();
    }
    print!("{buf}")
}
