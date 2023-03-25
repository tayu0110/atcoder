#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, mut m: usize, mut a: [usize; n]}

    a.sort();
    let res = a.into_iter().dedup().count() + m;

    println!("{}", std::cmp::min(res, n));
}
