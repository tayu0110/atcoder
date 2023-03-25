#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {s: Chars, q: usize, p: [(usize, usize); q]}

    for (mut t, k) in p {
        let mut k = k-1;
        let mut buf = vec![];

        while k > 0 && t > 0 {
            buf.push(k % 2);
            k >>= 1;
            t -= 1;
        }

        let mut now = s[k];
        if t > 0 {
            now = ((now as u8 - b'A' + (t % 3) as u8) % 3 + b'A') as char;
        }

        while let Some(b) = buf.pop() {
            if b == 0 {
                now = ((now as u8 - b'A' + 1) % 3 + b'A') as char;
            } else {
                now = ((now as u8 - b'A' + 2) % 3 + b'A') as char;
            }
        }

        println!("{}", now);
    }
}
