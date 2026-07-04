use std::{
    io::{stdin, Read},
    str::from_utf8_unchecked,
    sync::Mutex,
};

use cpio::putln;

static BUF: Mutex<[u8; 1 << 25]> = Mutex::new([0; 1 << 25]);

fn main() {
    let mut buf = BUF.lock().unwrap();
    let mut len = 0;
    let mut stdin = stdin().lock();
    while let Some(l) = stdin.read(&mut buf[len..]).ok().filter(|&len| len > 0) {
        len += l;
    }
    let buf = &buf[..len];
    let mut splitted = buf.split(|c| c.is_ascii_whitespace());
    let n = unsafe { from_utf8_unchecked(splitted.next().unwrap()) }
        .parse::<usize>()
        .unwrap();
    let q = unsafe { from_utf8_unchecked(splitted.next().unwrap()) }
        .parse::<usize>()
        .unwrap();
    let (mut sign, mut min, mut max) = (1, i32::MIN + 1, i32::MAX);
    for _ in 0..n {
        let s = splitted.next().unwrap();
        let p = unsafe { from_utf8_unchecked(splitted.next().unwrap()) }
            .parse::<i32>()
            .unwrap();
        if s.starts_with(b"N") {
            sign = -sign;
            min = -min;
            max = -max;
            (min, max) = (max, min);
        } else if s.ends_with(b"N") {
            max = max.min(p);
            min = min.min(p);
        } else {
            min = min.max(p);
            max = max.max(p);
        }
    }

    for _ in 0..q {
        let mut q = unsafe { from_utf8_unchecked(splitted.next().unwrap()) }
            .parse::<i32>()
            .unwrap();
        if sign < 0 {
            q = -q;
        }
        putln!(q.clamp(min, max));
    }
}
