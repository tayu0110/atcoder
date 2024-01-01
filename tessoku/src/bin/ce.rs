use proconio::*;
use std::io::{stdout, BufWriter, Write};

fn main() {
    input! {n: usize, mut a: [u32; n], q: usize}
    a.insert(0, 0);
    for i in 0..n {
        a[i + 1] += a[i];
    }

    let mut buf = String::with_capacity(q << 2);
    for _ in 0..q {
        input! {l: usize, r: usize}
        let sum = ((a[r] - a[l - 1]) as usize) << 1;
        if sum > r - l + 1 {
            buf.push_str("win\n");
        } else if sum == r - l + 1 {
            buf.push_str("draw\n");
        } else {
            buf.push_str("lose\n");
        }
    }

    BufWriter::new(stdout().lock())
        .write_all(buf.as_bytes())
        .ok();
}
