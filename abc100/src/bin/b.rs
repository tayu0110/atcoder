#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {d: usize, mut n: usize}

    if n == 100 {
        n += 1;
    }

    let mut res = n;
    for _ in 0..d {
        res *= 100;
    }
    println!("{}", res);
}
