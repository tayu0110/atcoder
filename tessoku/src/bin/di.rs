use std::io::{stdin, Read};

fn main() {
    let mut buf = [0; 1 << 20];
    let mut cur = &mut buf[..];
    let mut stdin = stdin().lock();
    while let Some(len) = stdin.read(cur).ok().filter(|&len| len > 0) {
        cur = &mut cur[len..];
    }
    let rem = cur.len();
    let len = buf.len() - rem;
    let mut buf = buf[..len].split(u8::is_ascii_whitespace);
    buf.next();
    let &(mut k) = buf.next().unwrap().last().unwrap();
    let s = buf.next().unwrap();
    for &b in s {
        k = (k + b) & 1;
    }
    if k == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
