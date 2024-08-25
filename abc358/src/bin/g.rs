use proconio::*;

fn main() {
    input! {h: usize, w: usize, k: usize, si: usize, sj: usize, a: [[usize; w]; h]}

    let mut t;
    let mut nt = vec![(si - 1, sj - 1, 0)];
    let mut res = 0;
    for i in 1..=(h * w).min(k) {
        let mut new = vec![vec![0; w]; h];
        while let Some((r, c, now)) = nt.pop() {
            for (dy, dx) in [(0, 1), (1, 0), (0, !0), (!0, 0), (0, 0)] {
                let nr = r.wrapping_add(dy);
                let nc = c.wrapping_add(dx);
                if nr < h && nc < w {
                    new[nr][nc] = new[nr][nc].max(now + a[nr][nc]);
                }
            }
        }

        t = new;
        let mut next = vec![];
        for r in 0..h {
            for c in 0..w {
                if t[r][c] > 0 {
                    res = res.max(t[r][c] + a[r][c] * (k - i));
                    next.push((r, c, t[r][c]));
                }
            }
        }

        nt = next;
    }

    println!("{res}")
}
