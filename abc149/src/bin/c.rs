#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {x: usize}
    let max = x + 10000;

    let mut d = vec![std::usize::MAX; max];
    for i in 2..max {
        if d[i] == std::usize::MAX {
            for j in (2..max).take_while(|j| i * *j < max) {
                d[i*j] = i;
            }
        }
    }

    for i in x..max {
        if d[i] == std::usize::MAX {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}
