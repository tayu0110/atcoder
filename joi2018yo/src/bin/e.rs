use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [[u32; w]; h]}

    let mut dp = vec![vec![vec![u32::MAX; w]; h]; h * w];
    dp[0][0][0] = 0;
    for i in 0..h * w - 1 {
        for r in 0..h {
            for c in 0..w {
                if dp[i][r][c] < u32::MAX {
                    for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                        let nr = r.wrapping_add(dr);
                        let nc = c.wrapping_add(dc);

                        if nr < h && nc < w {
                            let d = a[nr][nc] * (i * 2 + 1) as u32;
                            dp[i + 1][nr][nc] = dp[i + 1][nr][nc].min(dp[i][r][c] + d);
                        }
                    }
                }
            }
        }
    }

    println!("{}", dp.iter().map(|d| d[h - 1][w - 1]).min().unwrap());
}
