use proconio::*;

fn main() {
    input! {s: [marker::Bytes; 3]}
    let c = b'B' ^ b'R' ^ b'G' ^ b'H';
    println!("A{}C", s.iter().fold(c, |s, v| s ^ v[1]) as char)
}
