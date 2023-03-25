#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};

    for i in 1..=9 {
        if n % i == 0 && n / i < 10 {
            println!("Yes");
            std::process::exit(0);
        }
    }

    println!("No")
}
