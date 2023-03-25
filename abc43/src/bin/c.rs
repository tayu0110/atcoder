#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, a: [i32; n]}

    let mut res = std::i32::MAX;
    for i in -100..=100 {
        let sum = a.iter().map(|a| (*a - i) * (*a - i)).sum::<i32>();
        res = std::cmp::min(res, sum);
    }

    println!("{}", res);
}
