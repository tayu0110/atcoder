#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};

    let mut now = 0;
    for (i, v) in a.iter().enumerate() {
        if now + v > 2018 {
            println!("{}", i);
            std::process::exit(0);
        }
        now += *v;
    }

    println!("{}", n);
}
