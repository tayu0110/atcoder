use std::io::{stdin, BufReader, Read};

fn main() {
    let mut buf = String::new();
    BufReader::new(stdin().lock()).read_to_string(&mut buf).ok();

    let xor = buf
        .split_ascii_whitespace()
        .skip(3)
        .map(|s| unsafe { s.bytes().next_back().unwrap_unchecked() - b'0' })
        .map(|b| b.min(b.wrapping_sub(5)) >> 1)
        .fold(0, |s, v| s ^ v);

    if xor == 0 {
        println!("Second")
    } else {
        println!("First")
    }
}
