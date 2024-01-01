use std::{
    io::{stdin, BufReader, Read},
    mem::transmute,
};

fn main() {
    let mut buf = vec![];
    BufReader::new(stdin().lock()).read_to_end(&mut buf).ok();
    let buf = buf.as_slice();
    let buf = unsafe { transmute::<_, &str>(buf) };
    let xor = buf
        .split_ascii_whitespace()
        .skip(3)
        .map(|s| unsafe { s.parse::<u32>().unwrap_unchecked() })
        .fold(0, |s, v| s ^ (v - 1));

    if xor == 0 {
        println!("Second")
    } else {
        println!("First")
    }
}
