use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {h: usize, w: usize, r: usize, c: usize, s: [marker::Chars; h]}

    let mut map = [(0usize, 0usize); 256];
    map[b'^' as usize] = (0, 1);
    map[b'v' as usize] = (0, !0);
    map[b'<' as usize] = (1, 0);
    map[b'>' as usize] = (!0, 0);

    let mut res = vec![vec!['#'; w]; h];
    let mut nt = VecDeque::new();
    nt.push_back((r - 1, c - 1));
    while let Some((r, c)) = nt.pop_front() {
        if res[r][c] != '#' {
            continue;
        }
        res[r][c] = 'o';
        for (dx, dy) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);

            if nr < h && nc < w {
                if res[nr][nc] != '#' {
                    continue;
                }

                if s[nr][nc] == '#' {
                    continue;
                }

                if s[nr][nc] != '.' && map[s[nr][nc] as usize] != (dx, dy) {
                    continue;
                }

                nt.push_back((nr, nc));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' && res[i][j] == '#' {
                res[i][j] = 'x';
            }
        }
    }

    for res in res {
        println!("{}", res.iter().join(""))
    }
}
