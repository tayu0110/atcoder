use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: usize, mut l: [usize; n]};

    l.insert(0, 0);
    let res = l.into_iter().scan(0, |v, s| { *v += s; Some(*v) }).filter(|v| *v <= x).collect_vec();
    println!("{}", res.len());
}
