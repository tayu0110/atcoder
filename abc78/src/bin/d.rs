#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, _z: i32, w: i32, a: [i32; n]}

    if n == 1 {
        println!("{}", (a[0] - w).abs());
        std::process::exit(0);
    }

    let diff = (a[n-1] - a[n-2]).abs();

    if diff < (w - a[n-1]).abs() {
        println!("{}", (w - a[n-1]).abs());
    } else {
        println!("{}", diff);
    }
}
