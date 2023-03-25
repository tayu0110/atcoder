#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, x: usize, a: [usize; n]}
    let a = a.into_iter().map(|v| v - 1).collect_vec();

    let mut res = 0;
    let mut now = x - 1;
    let mut ck = vec![false; n];
    while !ck[now] {
        res += 1;
        ck[now] = true;
        now = a[now];
    }

    println!("{}", res);
}
