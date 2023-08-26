use std::collections::{HashMap, VecDeque};

use proconio::*;

fn main() {
    input! {h: usize, w: usize, t: usize, a: [marker::Chars; h]}

    let mut s = (0, 0);
    let mut g = (0, 0);
    let mut map = HashMap::new();
    let mut cnt = 1;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'o' {
                map.insert((i, j), cnt);
                cnt += 1;
            } else if a[i][j] == 'S' {
                s = (i, j);
            } else if a[i][j] == 'G' {
                g = (i, j);
            }
        }
    }

    map.insert(s, 0);
    map.insert(g, cnt);
    cnt += 1;

    let mut dist = vec![vec![std::usize::MAX; cnt]; cnt];
    for (&(r, c), &index) in &map {
        let mut nt = VecDeque::new();
        let mut ndist = vec![vec![std::usize::MAX; w]; h];
        nt.push_back((r, c, 0));
        while let Some((r, c, nd)) = nt.pop_front() {
            if ndist[r][c] != std::usize::MAX {
                continue;
            }
            ndist[r][c] = nd;

            if r > 0 && ndist[r - 1][c] == std::usize::MAX && a[r - 1][c] != '#' {
                nt.push_back((r - 1, c, nd + 1));
            }
            if r + 1 < h && ndist[r + 1][c] == std::usize::MAX && a[r + 1][c] != '#' {
                nt.push_back((r + 1, c, nd + 1));
            }
            if c > 0 && ndist[r][c - 1] == std::usize::MAX && a[r][c - 1] != '#' {
                nt.push_back((r, c - 1, nd + 1));
            }
            if c + 1 < w && ndist[r][c + 1] == std::usize::MAX && a[r][c + 1] != '#' {
                nt.push_back((r, c + 1, nd + 1));
            }
        }

        for (&(nr, nc), &nindex) in &map {
            dist[index][nindex] = ndist[nr][nc];
        }
    }

    let mut dp = vec![vec![std::usize::MAX; 1 << cnt]; cnt];
    dp[0][1] = 0;
    for i in 0..1 << cnt {
        for j in 0..cnt {
            if dp[j][i] == std::usize::MAX {
                continue;
            }

            for k in 0..cnt {
                if j == k {
                    continue;
                }

                if i & (1 << k) != 0 {
                    continue;
                }

                if dist[j][k] == std::usize::MAX {
                    continue;
                }

                if dp[j][i] + dist[j][k] > t {
                    continue;
                }

                let next = i | (1 << k);
                dp[k][next] = dp[k][next].min(dp[j][i] + dist[j][k]);
            }
        }
    }

    let mut res = -1;
    for i in 0..1 << cnt {
        if dp[cnt - 1][i] != std::usize::MAX {
            res = res.max(i.count_ones() as i32 - 2);
        }
    }

    println!("{}", res)
}
