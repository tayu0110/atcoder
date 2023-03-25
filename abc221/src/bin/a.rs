#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize}

    let mut res = 1;
    for _ in b+1..=a {
        res *= 32;
    }

    println!("{}", res);
}
