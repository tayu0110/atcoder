use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, si: usize, sj: usize, c: [marker::Chars; h], x: String}

    let (mut y, mut j) = (si - 1, sj - 1);
    let mut map = HashMap::new();
    map.insert('L', (0usize, !0usize));
    map.insert('R', (0, 1));
    map.insert('U', (!0, 0));
    map.insert('D', (1, 0));
    for x in x.chars() {
        let (dy, dx) = map.get(&x).unwrap();
        let nr = y.wrapping_add(*dy);
        let nc = j.wrapping_add(*dx);
        if nr < h && nc < w && c[nr][nc] == '.' {
            (y, j) = (nr, nc);
        }
    }

    println!("{} {}", y + 1, j + 1);
}
