#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [i64; n]};

    let mut sum = 0;
    let mut max = 0;
    let mut now = 0;
    let mut res = 0;
    for i in 0..n {
        sum += a[i];
        max = std::cmp::max(max, sum);
        res = std::cmp::max(res, now + max);
        now += sum;
    }

    println!("{}", res);
}
