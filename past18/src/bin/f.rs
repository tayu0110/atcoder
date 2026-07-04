use std::{
    io::{stdin, Read},
    str::from_utf8_unchecked,
};

fn main() {
    let mut buffer = [0u8; 600];
    let mut buf = &mut buffer[..];
    let mut lock = stdin().lock();
    while let Some(len) = lock.read(buf).ok().filter(|&len| len > 0) {
        buf = &mut buf[len..];
    }

    let rem = buf.len();
    let mut buf = buffer[..buffer.len() - rem].split(|c| c.is_ascii_whitespace());
    let s = buf.next().unwrap();
    buf.next();

    let mut map = [0; 128];
    while let Some(c) = buf.next().filter(|c| !c.is_empty()) {
        let v = unsafe { from_utf8_unchecked(buf.next().unwrap()) }
            .parse::<i16>()
            .unwrap();
        map[c[0] as usize] = v;
    }
    for c in b'0'..=b'9' {
        map[c as usize] = c as i16 - b'0' as i16;
    }

    let mut res = map[s[0] as usize];
    for chunk in s[1..].chunks_exact(2) {
        match chunk {
            [b'+', c, ..] => res += map[*c as usize],
            [b'-', c, ..] => res -= map[*c as usize],
            _ => unreachable!(),
        }
    }

    println!("{res}")
}
