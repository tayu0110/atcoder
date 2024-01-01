use proconio::*;
use std::io::{stdout, Write};

const RES: [[u8; 11]; 1025] = {
    let mut buf = [[b'\n'; 11]; 1025];
    let mut i = 0usize;
    while i < 1024 {
        let mut n = (i as u32).wrapping_shl(22).reverse_bits();
        let mut res = [b'\n'; 11];
        let mut j = 0;
        while j < 10 {
            if n & 1 == 0 {
                res[j] = b'4';
            } else {
                res[j] = b'7';
            }
            n >>= 1;
            j += 1;
        }
        i += 1;
        buf[i] = res;
    }

    buf
};

fn main() {
    input! {n: u16}
    stdout().lock().write_all(&RES[n as usize]).ok();
}
