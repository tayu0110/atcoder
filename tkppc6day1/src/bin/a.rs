#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize}

    if n < 2015 || n == 2017 {
        println!("-1");
    } else if 2015 <= n && n < 2017 {
        println!("{}", n - 2014);
    } else {
        println!("{}", n - 2015);
    }
}
