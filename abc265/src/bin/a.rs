#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize, y: usize, n: usize};

    if 3 * x <= y {
        println!("{}", n * x);
    } else {
        println!("{}", n / 3 * y + n % 3 * x);
    }
}
