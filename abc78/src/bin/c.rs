#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize}

    println!("{}", (1900 * m + 100 * (n - m)) * 2usize.pow(m as u32));
}
