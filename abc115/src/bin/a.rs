#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {d: usize}

    print!("Christmas");

    if d == 24 {
        print!(" Eve");
    } else if d == 23 {
        print!(" Eve Eve");
    } else if d == 22 {
        print!(" Eve Eve Eve");
    }

    println!("")
}
