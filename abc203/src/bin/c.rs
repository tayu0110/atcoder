#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, mut k: usize, mut p: [(usize, usize); n]}

    p.sort();

    let mut now = 0;

    for (a, b) in p {
        if a - now > k {
            println!("{}", now + k);
            std::process::exit(0);
        }

        k -= a - now;
        k += b;
        now = a;
    }

    if k > 0 {
        println!("{}", now + k);
    }
}
