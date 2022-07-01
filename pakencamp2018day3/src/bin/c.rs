#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {b: usize};

    let mut check = vec![0; b+1];
    for i in 2..b {
        let mut now = i;
        while now < b {
            now = now + now / 2;
        }

        if now == b {
            check[i] = 1;
        }
    }

    println!("{}", check.into_iter().fold(0, |sum, x| sum + x));
}
