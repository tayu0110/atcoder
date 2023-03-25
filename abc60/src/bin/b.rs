#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {a: usize, b: usize, c: usize}

    let g = gcd(a, b);

    if c % g == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
