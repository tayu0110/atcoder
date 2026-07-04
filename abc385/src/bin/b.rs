use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {h: usize, _: usize, x: usize, y: usize, s: [marker::Bytes; h], t: marker::Bytes}

    let mut set = HashSet::new();
    let (mut r, mut c) = (x - 1, y - 1);
    for t in t {
        let (nr, nc) = match t {
            b'U' => (r - 1, c),
            b'D' => (r + 1, c),
            b'L' => (r, c - 1),
            b'R' => (r, c + 1),
            _ => unreachable!(),
        };

        if s[nr][nc] == b'.' {
            (r, c) = (nr, nc);
        } else if s[nr][nc] == b'@' {
            (r, c) = (nr, nc);
            set.insert((nr, nc));
        }
    }

    println!("{} {} {}", r + 1, c + 1, set.len())
}
