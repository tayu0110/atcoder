#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, a: [[usize; m]; n]}

    let mut res = 0;
    for i in 0..m {
        for j in i+1..m {
            let mut score = 0;
            for k in 0..n {
                score += std::cmp::max(a[k][i], a[k][j]);
            }
            res = std::cmp::max(res, score);
        }
    }

    println!("{}", res);
}
