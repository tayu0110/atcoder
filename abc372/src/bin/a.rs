use proconio::*;

fn main() {
    input! {s: marker::Chars}
    println!("{}", s.iter().filter(|c| c != &&'.').collect::<String>())
}
