use std::{
    io::{stdin, Read},
    str::from_utf8_unchecked,
    sync::Mutex,
};

use cpio::putln;

static BUF: Mutex<[i32; 500001]> = Mutex::new([0; 500001]);
static INPUT: Mutex<[u8; 1 << 25]> = Mutex::new([0; 1 << 25]);

fn main() {
    let mut input = INPUT.lock().unwrap();
    let mut buf = &mut input[..];
    let mut len = 0;
    let mut stdin = stdin().lock();
    while let Some(l) = stdin.read(buf).ok().filter(|&len| len > 0) {
        len += l;
        buf = &mut buf[l..];
    }
    let mut split = input[..len].split(u8::is_ascii_whitespace);
    let n = split
        .next()
        .and_then(|s| unsafe { from_utf8_unchecked(s) }.parse::<usize>().ok())
        .unwrap();

    let mut cum = BUF.lock().unwrap();
    for (i, a) in split.enumerate().take(n) {
        let mut a = a.iter().fold(0, |s, v| s * 10 + (*v - b'0') as u32);
        a += cum[i] as u32;
        let start = i + 1;
        let end = n.min(start + a as usize);
        let num = end - start;
        a -= num as u32;
        cum[start] += 1;
        cum[end] -= 1;
        cum[i + 1] += cum[i];
        putln!(a);
    }
}
