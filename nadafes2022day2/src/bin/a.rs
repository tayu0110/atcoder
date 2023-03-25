#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    input! {n: usize, k: usize, m: usize};
    let l = lcm(k, m) / k;
    if n % l == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
