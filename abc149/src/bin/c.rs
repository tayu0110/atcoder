#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    source::line::LineSource,
};

fn main() {
    input! {x: usize}
    let max = x + 10000;

    let mut d = vec![std::usize::MAX; max];
    for i in 2..max {
        if d[i] == std::usize::MAX {
            for j in (2..max).take_while(|j| i * *j < max) {
                d[i * j] = i;
            }
        }
    }

    for (i, d) in d.iter().enumerate().take(max).skip(x) {
        if d == &usize::MAX {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}
