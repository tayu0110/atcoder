use proconio::*;

fn main() {
    input! {mut s: String}
    let n = s.len();
    s.remove(n / 2);
    println!("{}", s)
}
