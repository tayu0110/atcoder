#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}

    let m = n * 108 / 100;
    if m < 206 {
        println!("Yay!");
    } else if m == 206 {
        println!("so-so");
    } else {
        println!(":(");
    }
}
