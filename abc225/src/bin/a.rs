use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s.sort();
    s.dedup();
    if s.len() == 3 {
        println!("6")
    } else if s.len() == 2 {
        println!("3")
    } else {
        println!("1")
    }
}
