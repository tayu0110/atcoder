use std::io::{stdout, Write};

use proconio::*;

const BUF: [u8; 100000] = {
    let mut buf = [b'0'; 100000];
    buf[0] = b'1';
    buf
};

fn main() {
    input! {n: usize}
    stdout().lock().write_all(&BUF[..n]).ok();
}
