#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n-1]}

    for i in 0..n {
        let mut now = 0;

        for &(c, s, f) in p.iter().skip(i) {
            if now < s {
                now = s;
            }
            now = (now + f - 1) / f * f + c;
        }

        println!("{}", now);
    }
}
