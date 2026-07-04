use proconio::*;

fn main() {
    input! {s: String, n: usize}
    println!("{}", &s[n..s.len() - n])
}
