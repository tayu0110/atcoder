use std::io::Write;

use itertools::Itertools;
use proconio::*;

fn main() {
    input_interactive!(n: usize);

    let m = n.next_power_of_two().trailing_zeros();
    println!("{m}");
    std::io::stdout().flush().unwrap();
    let mut width = 1;
    for _ in 0..m {
        let mut buf = vec![];
        let mut now = 1;
        while now <= n {
            for _ in 0..width {
                if now <= n {
                    buf.push(now);
                    now += 1;
                }
            }
            now += width;
        }

        assert!(buf.iter().all(|&a| 1 <= a && a <= n));

        println!("{} {}", buf.len(), buf.iter().join(" "));
        std::io::stdout().flush().unwrap();
        width <<= 1;
    }

    input_interactive!(mut s: marker::Bytes);

    let (mut l, mut r) = (0, n.next_power_of_two());
    while let Some(c) = s.pop() {
        let m = (r + l) / 2;
        if c == b'1' {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", l + 1);
    std::io::stdout().flush().unwrap();
}
