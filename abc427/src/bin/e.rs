use std::collections::VecDeque;

use proconio::*;

const N: usize = 36;
const M: usize = N / 3;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut map = [[b'.'; N]; N];
    let mut start = (0, 0);
    let mut offset = (0, 0);
    let mut cum = [[0; 37]; 37];
    for i in 0..h {
        for j in 0..w {
            map[12 + i][12 + j] = s[i][j];
            if s[i][j] == b'T' {
                start = (i + 12, j + 12);
                offset = (i, j);
            } else if s[i][j] == b'#' {
                cum[13 + i][13 + j] = 1;
            }
        }
    }

    for i in 0..=N {
        for j in 0..N {
            cum[i][j + 1] += cum[i][j];
        }
    }
    for j in 0..=N {
        for i in 0..N {
            cum[i + 1][j] += cum[i][j];
        }
    }

    let mut nt = VecDeque::new();
    nt.push_back((start.0, start.1, 0, 0, 0, 0, 0));
    let mut checked = vec![vec![vec![vec![vec![vec![false; M]; M]; M]; M]; N]; N];
    checked[start.0][start.1][0][0][0][0] = true;
    while let Some((y, x, l, r, u, d, dist)) = nt.pop_front() {
        let (or, oc) = offset;
        if l + r >= w || u + d >= h {
            println!("{dist}");
            return;
        }
        if y >= or && x >= oc {
            let cr = y - or;
            let cc = x - oc;
            let sum = cum[cr + h - d][cc + w - r] + cum[cr + u][cc + l]
                - cum[cr + h - d][cc + l]
                - cum[cr + u][cc + w - r];
            eprintln!(
                "cr: {cr}, cc: {cc}, l: {l}, r: {r}, u: {u}, d: {d}, dist: {dist}, sum: {sum}"
            );
            if sum == 0 {
                println!("{dist}");
                return;
            }
        }

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let ny = y.wrapping_add(dr);
            let nx = x.wrapping_add(dc);
            if ny >= map.len() || nx >= map[0].len() {
                continue;
            }
            let nl = l.max(nx.saturating_sub(start.1));
            let nr = r.max(start.1.saturating_sub(nx));
            let nu = u.max(ny.saturating_sub(start.0));
            let nd = d.max(start.0.saturating_sub(ny));

            if checked[ny][nx][nl][nr][nu][nd] {
                continue;
            }
            if map[ny][nx] == b'#' {
                continue;
            }

            nt.push_back((ny, nx, nl, nr, nu, nd, dist + 1));
            checked[ny][nx][nl][nr][nu][nd] = true;
        }
    }

    println!("-1")
}
