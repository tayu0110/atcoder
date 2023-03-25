#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: [Chars; 10]}

    let (mut a, mut b, mut c, mut d) = (20, 0, 20, 0);
    for i in 0..10 {
        for (j, nc) in s[i].iter().enumerate() {
            if nc == &'#' {
                a = std::cmp::min(a, i);
                b = std::cmp::max(b, i);
                c = std::cmp::min(c, j);
                d = std::cmp::max(d, j);
            }
        }
    }

    println!("{} {}", a+1, b+1);
    println!("{} {}", c+1, d+1);
}
