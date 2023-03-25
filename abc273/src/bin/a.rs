#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize}

    let mut res = 1;
    for i in 1..=n {
        res *= i;
    }

    println!("{}", res);
}
