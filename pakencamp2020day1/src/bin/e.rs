#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize, y: usize};

    if x == y {
        if x == 0 {
            println!("1 1");
        } else {
            println!("-1");
        }
    } else {
        if x < y {
            if x == 0 {
                println!("{} {}", 2*y, y);
            } else {
                println!("{} {}", x + y, y);
            }
        } else {
            if y == 0 {
                println!("{} {}", x, 2*x);
            } else {
                println!("{} {}", x, x + y);
            }
        }
    }
}
