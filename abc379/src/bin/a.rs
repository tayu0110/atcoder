use proconio::*;

fn main() {
    input! {n: marker::Chars}
    println!("{}{}{} {}{}{}", n[1], n[2], n[0], n[2], n[0], n[1])
}
