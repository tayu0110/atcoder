use proconio::*;

fn main() {
    input! {l: usize, s: String}
    println!("{}", &s[..l.min(s.len())])
}
