use proconio::*;

fn main() {
    input! {s: String}
    println!("{}4", &s[..s.len() - 1]);
}
