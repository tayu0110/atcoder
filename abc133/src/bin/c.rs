#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {l: usize, r: usize};

    if r - l > 2500 {
        println!("0");
    } else {
        let mut res = 1000000000;
        for i in l..=r {
            for j in i+1..=r {
                res = std::cmp::min(res, (i * j) % 2019);
            }
        }

        println!("{}", res);
    }
}
