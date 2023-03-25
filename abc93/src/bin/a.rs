use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s.sort();

    if s == vec!['a', 'b', 'c'] {
        println!("Yes")
    } else {
        println!("No")
    }
}
