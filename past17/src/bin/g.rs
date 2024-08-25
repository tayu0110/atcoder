use proconio::*;

fn solve(r: usize, c: usize, now: usize, g: &[Vec<u8>], s: &[u8], memo: &mut [Vec<bool>]) -> bool {
    if now == s.len() {
        return true;
    }

    if s[now] != g[r][c] {
        return false;
    }

    let mut res = false;
    memo[r][c] = true;
    for dy in [0, 1, !0] {
        for dx in [0, 1, !0] {
            if dy == 0 && dx == 0 {
                continue;
            }
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);

            if nr < g.len() && nc < g[0].len() && !memo[nr][nc] {
                res |= solve(nr, nc, now + 1, g, s, memo);
            }
        }
    }

    memo[r][c] = false;
    res
}

fn main() {
    input! {h: usize, w: usize, g: [marker::Bytes; h], _: usize, s: marker::Bytes}

    let mut memo = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if solve(i, j, 0, &g, &s, &mut memo) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
