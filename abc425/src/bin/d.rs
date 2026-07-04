use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut map = vec![vec![0u8; w]; h];
    let mut nt = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                map[i][j] = u8::MAX;
                nt.push((i, j));
            }
        }
    }

    let mut res = 0;
    while !nt.is_empty() {
        res += nt.len();
        let mut set = FxHashSet::default();
        while let Some((r, c)) = nt.pop() {
            for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr < h && nc < w && map[nr][nc] < u8::MAX {
                    map[nr][nc] += 1;
                    set.insert((nr, nc));
                }
            }
        }

        for (r, c) in set {
            if map[r][c] == 1 {
                nt.push((r, c));
                map[r][c] = u8::MAX;
            }
        }
    }

    println!("{}", res)
}
