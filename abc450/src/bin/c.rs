use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut cnt = 0;
    let mut t = vec![vec![-1; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'.' && t[i][j] < 0 {
                let mut nt = vec![(i, j)];
                t[i][j] = cnt;
                while let Some((r, c)) = nt.pop() {
                    for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                        let nr = r.wrapping_add(dr);
                        let nc = c.wrapping_add(dc);
                        if nr < h && nc < w && s[nr][nc] == b'.' && t[nr][nc] < 0 {
                            t[nr][nc] = cnt;
                            nt.push((nr, nc));
                        }
                    }
                }
                cnt += 1;
            }
        }
    }

    let mut check = vec![false; cnt as usize];
    for i in 0..h {
        if t[i][0] >= 0 {
            check[t[i][0] as usize] = true;
        }
        if t[i][w - 1] >= 0 {
            check[t[i][w - 1] as usize] = true;
        }
    }
    for j in 0..w {
        if t[0][j] >= 0 {
            check[t[0][j] as usize] = true;
        }
        if t[h - 1][j] >= 0 {
            check[t[h - 1][j] as usize] = true;
        }
    }

    println!("{}", check.into_iter().filter(|&b| !b).count())
}
