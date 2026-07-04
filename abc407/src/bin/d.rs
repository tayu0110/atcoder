use proconio::*;

fn solve(a: &[Vec<usize>], cur: &mut [Vec<bool>], memo: &mut [usize]) {
    let h = cur.len();
    let w = cur[0].len();
    let mut mask = 0;
    for i in 0..h {
        for j in 0..w {
            if cur[i][j] {
                mask |= 1 << (i * w + j);
            }
        }
    }

    if memo[mask] != usize::MAX {
        return;
    }

    let mut score = 0;
    for i in 0..h * w {
        if mask & (1 << i) == 0 {
            let r = i / w;
            let c = i % w;
            score ^= a[r][c];
        }
    }
    memo[mask] = score;
    for i in 0..h {
        for j in 0..w {
            if cur[i][j] {
                continue;
            }

            if i + 1 < h && !cur[i + 1][j] {
                cur[i][j] = true;
                cur[i + 1][j] = true;
                solve(a, cur, memo);
                cur[i + 1][j] = false;
                cur[i][j] = false;
            }

            if j + 1 < w && !cur[i][j + 1] {
                cur[i][j] = true;
                cur[i][j + 1] = true;
                solve(a, cur, memo);
                cur[i][j + 1] = false;
                cur[i][j] = false;
            }
        }
    }
}

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    let mut memo = vec![usize::MAX; 1 << (h * w)];
    solve(&a, &mut vec![vec![false; w]; h], &mut memo);
    let mut res = 0;
    for i in 0..1 << h * w {
        if memo[i] < usize::MAX {
            res = res.max(memo[i]);
        }
    }

    println!("{res}")
}
