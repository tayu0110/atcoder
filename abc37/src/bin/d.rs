use proconio::*;

fn dfs(r: usize, c: usize, a: &[Vec<usize>], res: &mut [Vec<usize>]) -> usize {
    if res[r][c] > 0 {
        return res[r][c];
    }

    let (h, w) = (a.len(), a[0].len());
    let mut t = 1;
    for (dy, dx) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let (nr, nc) = (r.wrapping_add(dy), c.wrapping_add(dx));

        if nr < h && nc < w && a[r][c] < a[nr][nc] {
            t += dfs(nr, nc, a, res);
            t %= 1000000007;
        }
    }

    res[r][c] = t;
    t
}

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    let mut res = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if res[i][j] > 0 {
                continue;
            }

            dfs(i, j, &a, &mut res);
        }
    }

    println!(
        "{}",
        res.iter().flatten().fold(0, |s, v| (s + v) % 1000000007)
    )
}
