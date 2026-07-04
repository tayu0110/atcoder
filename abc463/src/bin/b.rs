use proconio::*;

fn main() {
    input! {n: usize, x: char, s: [marker::Bytes; n]}

    let x = (x as u8 - b'A') as usize;
    if s.iter().any(|s| s[x] == b'o') {
        println!("Yes")
    } else {
        println!("No")
    }
}
