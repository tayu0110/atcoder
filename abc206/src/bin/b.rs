#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    let mut now = 0usize;

    for i in 1..=n {
        now += i;

        if now >= n {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}
