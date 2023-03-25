#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, k: usize}

    let mut res = 0;
    for i in 1..=n {
        for j in 1..=k {
            res += i * 100 + j;
        }
    }

    println!("{}", res);
}
