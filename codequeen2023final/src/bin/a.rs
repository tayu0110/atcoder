use proconio::*;

fn main() {
    input! {s: String, c: char}
    println!("{}", s.replace(c, &format!("{}{}", c, c)))
}
