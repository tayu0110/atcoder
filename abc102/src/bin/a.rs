#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    if n % 2 == 0 {
        println!("{}", n);
    } else {
        println!("{}", n*2);
    }
}
