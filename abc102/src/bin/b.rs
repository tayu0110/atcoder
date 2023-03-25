#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut res = 0;
    for i in 0..n {
        for j in i+1..n {
            res = std::cmp::max(res, (a[i]-a[j]).abs());
        }
    }

    println!("{}", res);
}
