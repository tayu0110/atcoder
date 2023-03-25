#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, y: usize}

    for i in 0..=n {
        for j in (0..=n).take_while(|j| i + *j <= n) {
            let k = n - i - j;
            if 10000 * i + 5000 * j + 1000 * k == y {
                println!("{} {} {}", i, j, k);
                std::process::exit(0);
            }
        }
    }

    println!("-1 -1 -1")
}
