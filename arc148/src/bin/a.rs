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
    input! {n: usize, mut a: [usize; n]};
    a.sort();

    let b = a[0];
    for i in 0..n {
        a[i] -= b;
    }

    let g = a.iter().fold(0, |s, v| gcd(*v, s));

    if g != 1 {
        println!("1");
    } else {
        println!("2");
    }
}
