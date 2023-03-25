#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut x: i128, y: i128, a: i128, b: i128}

    let mut res = 0;
    while (a - 1) * x < b && a * x < y {
        res += 1;
        x *= a;
    }

    res += (y - 1 - x) / b;

    println!("{}", res);
}
