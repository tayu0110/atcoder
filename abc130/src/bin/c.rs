#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {w: usize, h: usize, x: usize, y: usize};

    print!("{} ", (w * h) as f64 / 2.0);
    if w % 2 == 0 && h % 2 == 0 && w / 2 == x && h / 2 == y {
        println!("1");
    } else {
        println!("0");
    }
}
