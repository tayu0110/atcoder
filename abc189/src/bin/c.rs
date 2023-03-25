#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    for i in 0..n {
        let mut min = 0x3f3f3f3f3f3f3f3f;
        for j in i..n {
            min = std::cmp::min(min, a[j]);
            res = std::cmp::max(res, min * (j - i + 1));
        }
    }

    println!("{}", res);
}
