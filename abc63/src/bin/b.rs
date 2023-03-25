use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    let len = s.len();
    s.sort();
    s.dedup();

    if s.len() == len {
        println!("yes")
    } else {
        println!("no")
    }
}
