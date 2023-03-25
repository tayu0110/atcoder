#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, c: usize}

    if c == 0 {
        if a > b {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        if b > a {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
