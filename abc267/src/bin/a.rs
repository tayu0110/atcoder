#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: String};

    if s == "Monday" {
        println!("5");
    } else if s == "Tuesday" {
        println!("4");
    } else if s == "Wednesday" {
        println!("3");
    } else if s == "Thursday" {
        println!("2");
    } else {
        println!("1");
    }
}
