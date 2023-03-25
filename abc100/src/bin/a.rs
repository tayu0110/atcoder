#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut a: usize, mut b: usize}
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    let rem = 16 - b * 2;

    if rem >= (a-b) * 2 {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
