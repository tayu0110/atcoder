#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: usize, k: usize}

    let mut res = x;
    for i in 0..k {
        for _ in 0..i {
            res /= 10;
        }

        if res % 10 > 4 {
            res = (res / 10 + 1) * 10;
        } else {
            res = (res / 10) * 10;
        }

        for _ in 0..i {
            res *= 10;
        }
    }

    println!("{}", res);
}
