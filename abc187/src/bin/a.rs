use proconio::*;

fn main() {
    input! {a: marker::Bytes, b: marker::Bytes}
    let na = a.iter().map(|&c| c as u16).sum::<u16>();
    let nb = b.iter().map(|&c| c as u16).sum::<u16>();
    println!("{}", na.max(nb) - b'0' as u16 * 3)
}
