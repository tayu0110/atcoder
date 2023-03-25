use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: [String; n]};

    let black = s.into_iter().filter(|s| s == "black").collect_vec().len();
    if black * 2 > n {
        println!("black");
    } else {
        println!("white");
    }
}
