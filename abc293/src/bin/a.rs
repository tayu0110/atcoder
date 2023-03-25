use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    let n = s.len();

    for i in 0..n / 2 {
        s.swap(i * 2, i * 2 + 1)
    }

    println!("{}", s.into_iter().collect::<String>())
}
