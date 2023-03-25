#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut k: usize, a: usize}

    let mut now = a;
    while k > 1 {
        k -= 1;
        now += 1;
        if now > n {
            now = 1;
        }
    }

    println!("{}", now);
}
