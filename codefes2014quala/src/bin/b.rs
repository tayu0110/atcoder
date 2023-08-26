use proconio::*;

fn main() {
    input! {a: marker::Chars, b: usize}
    let i = (b - 1) % a.len();
    println!("{}", a[i])
}
