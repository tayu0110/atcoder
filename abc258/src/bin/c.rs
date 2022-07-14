#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, s: Chars, p: [(usize, usize); q]};

    let mut idx = 0;
    for (t, x) in p {
        if t == 1 {
            idx = (idx + n - x) % n;
        } else {
            println!("{}", s[(idx + x-1) % n]);
        }
    }
}
