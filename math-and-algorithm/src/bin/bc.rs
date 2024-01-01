use std::io::{stdin, BufReader, Read};

fn main() {
    let mut buf = String::new();
    let mut stdin = BufReader::new(stdin().lock());
    stdin.read_to_string(&mut buf).ok();
    let mut s = buf.split_ascii_whitespace();
    let (_, k) = (s.next(), s.next().unwrap().parse::<u16>().unwrap());
    let sum = s.fold(k, |s, v| s.wrapping_sub(v.parse().unwrap()));
    const MASK: u16 = (1 << 15) | 1;
    if sum & MASK != 0 {
        println!("No")
    } else {
        println!("Yes")
    }
}
