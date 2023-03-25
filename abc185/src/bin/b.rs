#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize, m: usize, t: usize, p: [(usize, usize); m]}

    let max = n;
    let mut now = 0;
    for (a, b) in p {
        if a - now >= n {
            println!("No");
            std::process::exit(0);
        }
        n -= a - now;
        n += b - a;
        n = std::cmp::min(n, max);
        now = b;
    }

    if t - now >= n {
        println!("No");
    } else {
        println!("Yes");
    }
}
