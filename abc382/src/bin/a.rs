use proconio::*;

fn main() {
    input! {n: usize, d: usize, s: marker::Bytes}
    println!("{}", n - (s.into_iter().filter(|&c| c == b'@').count() - d))
}
