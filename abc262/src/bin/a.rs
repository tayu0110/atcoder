#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {y: usize};

    for i in y..=3100 {
        if i % 4 == 2 {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}
