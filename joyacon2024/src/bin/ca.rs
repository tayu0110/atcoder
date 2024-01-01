use proconio::*;

fn main() {
    input! {_: String, s: String, _: String}

    println!("A{}C", s.chars().next().unwrap())
}
