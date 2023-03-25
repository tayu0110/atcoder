#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize};

    println!("{}", (2 * a + 100).saturating_sub(b));
}
