#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: i64, b: i64, c: i64, d: i64}

    if c * d - b <= 0 {
        println!("-1");
        std::process::exit(0);
    }

    println!("{}", (a + (c * d - b) - 1) / (c * d - b));
}
