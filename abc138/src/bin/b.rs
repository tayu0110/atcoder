#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};

    let mut res = 0.0;
    for v in a {
        res += 1.0 / v as f64;
    }
    println!("{}", 1.0 / res);
}
