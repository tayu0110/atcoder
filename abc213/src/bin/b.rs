#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    let mut a = a.into_iter().enumerate().map(|(l, r)| (r, l)).collect_vec();

    a.sort();
    println!("{}", a[n-2].1 + 1);
}
