#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, x: usize}

    if x < a || a + b < x {
        println!("NO");
    } else {
        println!("YES");
    }
}
