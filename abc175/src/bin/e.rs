use proconio::input;

fn main() {
    input! {h: usize, w: usize, k: usize, p: [(usize, usize, usize); k]}

    let map = {
        let mut map = vec![vec![0; w]; h];
        for (r, c, v) in p {
            map[r - 1][c - 1] = v;
        }
        map
    };

    let mut memo = vec![vec![vec![0; 4]; w]; h];
    memo[0][0][1] = map[0][0];
    for i in 0..h {
        for j in 0..w {
            for r in 0..=3 {
                if i + 1 < h {
                    memo[i + 1][j][0] = std::cmp::max(memo[i + 1][j][0], memo[i][j][r]);
                    memo[i + 1][j][1] =
                        std::cmp::max(memo[i + 1][j][1], memo[i][j][r] + map[i + 1][j]);
                }
                if j + 1 < w {
                    memo[i][j + 1][r] = std::cmp::max(memo[i][j + 1][r], memo[i][j][r]);
                    if r < 3 {
                        memo[i][j + 1][r + 1] =
                            std::cmp::max(memo[i][j + 1][r + 1], memo[i][j][r] + map[i][j + 1]);
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..=3 {
        res = std::cmp::max(res, memo[h - 1][w - 1][i]);
    }

    println!("{}", res);
}
