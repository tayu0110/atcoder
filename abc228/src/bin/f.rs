#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {h: usize, w: usize, h1: usize, w1: usize, h2: usize, w2: usize, a: [[usize; w]; h]}

    let (h2, w2) = (std::cmp::min(h1, h2), std::cmp::min(w1, w2));

    let mut sum = vec![vec![0; w+1]; h+1];
    for i in 0..h {
        for j in 0..w {
            sum[i+1][j+1] = a[i][j];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            sum[i][j] += sum[i][j-1];
            sum[i][j] += sum[i-1][j];
        }
    }

}
