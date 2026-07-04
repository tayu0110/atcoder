use proconio::*;

fn main() {
    input! {_: usize, a: usize, b: usize, s: String}
    println!("{}", &s[a..s.len() - b]);
}
