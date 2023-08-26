use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s.rotate_left(1);
    println!("{}", s.into_iter().collect::<String>())
}
