#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: String};

    let t = s.clone().chars().rev().collect::<String>();
    let f = s.find('A').unwrap();
    let l = t.find('Z').unwrap();

    println!("{}", s.len()-l - f);
}
