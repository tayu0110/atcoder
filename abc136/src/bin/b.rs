use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};

    let res = (1..=n).into_iter().filter(|v| v.to_string().len() % 2 == 1).collect_vec().len();

    println!("{}", res);
}
