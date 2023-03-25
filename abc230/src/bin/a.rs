#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize}

    if n >= 42 {
        n += 1;
    }

    println!("AGC{:03}", n);
}
