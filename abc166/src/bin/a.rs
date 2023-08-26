use proconio::*;

fn main() {
    input! {mut s: marker::Bytes}

    s[1] ^= b'R' ^ b'B';
    println!("{}", std::str::from_utf8(&s).unwrap());
}
