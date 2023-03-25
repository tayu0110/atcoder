use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, k: usize};

    let v = (1..110).into_iter().filter(|i| a % i == 0 && b % i == 0).collect_vec();

    println!("{}", v[v.len()-k]);
}
