use proconio::*;

fn main() {
    input! {n: usize, s: String}
    if s.ends_with("G") {
        println!("{}", &s[..n - 1])
    } else {
        println!("{}G", s)
    }
}
