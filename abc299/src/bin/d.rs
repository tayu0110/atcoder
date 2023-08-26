use proconio::*;
use std::io::{self, *};

fn main() {
    let mut stdin = source::line::LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n: usize}

    let (mut l, mut r) = (1, n);
    while r - l > 1 {
        let m = (r + l) / 2;
        println!("? {}", m);

        input! {s: usize}
        if s == 0 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("! {}", l);
}
