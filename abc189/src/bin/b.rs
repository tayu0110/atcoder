#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: usize, p: [(usize, usize); n]}
    let x = x * 100;

    let mut now = 0;
    for (i, (v, p)) in p.into_iter().enumerate() {
        now += v * p;
        if now > x {
            println!("{}", i+1);
            std::process::exit(0);
        }
    }

    println!("-1");
}
