use proconio::input;

fn flip(s: &Vec<u32>) -> Vec<u32> {
    s.iter().cloned().map(|v| 1 - v).collect()
}

fn main() {
    input! {h: usize, w: usize, a: [[u32; w]; h]}
    const INF: usize = std::usize::MAX >> 10;

    let mut dp = vec![vec![vec![INF; 2]; 2]; h];
    for i in 0..2 {
        let s = if i == 0 { a[0].clone() } else { flip(&a[0]) };
        'mid: for j in 0..2 {
            let t = if j == 0 { a[1].clone() } else { flip(&a[1]) };
            for k in 0..w {
                if k > 0 && s[k] == s[k - 1] {
                    continue;
                }
                if k + 1 < w && s[k] == s[k + 1] {
                    continue;
                }
                if s[k] == t[k] {
                    continue;
                }

                continue 'mid;
            }
            dp[1][i][j] = i + j;
        }
    }

    let mut pprev = flip(&a[0]);
    let mut prev = flip(&a[1]);
    for i in 2..h {
        let (s, t, u) = (&a[i - 2], &a[i - 1], &a[i]);
        let (rs, rt, ru) = (&pprev, &prev, flip(&a[i]));
        for j in 0..2 {
            let s = if j == 0 { s } else { rs };
            for k in 0..2 {
                let t = if k == 0 { t } else { rt };
                for l in 0..2 {
                    let u = if l == 0 { u } else { &ru };
                    let mut bad = false;
                    for m in 0..w {
                        if m > 0 && t[m] == t[m - 1] {
                            continue;
                        }
                        if m + 1 < w && t[m] == t[m + 1] {
                            continue;
                        }
                        if s[m] == t[m] {
                            continue;
                        }
                        if t[m] == u[m] {
                            continue;
                        }

                        bad = true;
                        break;
                    }

                    if bad {
                        continue;
                    }

                    dp[i][k][l] = std::cmp::min(dp[i][k][l], dp[i - 1][j][k] + l);
                }
            }
        }

        pprev = prev;
        prev = ru;
    }

    let mut res = INF;
    for i in 0..2 {
        for j in 0..2 {
            let s = if i == 0 { &a[h - 2] } else { &pprev };
            let t = if j == 0 { &a[h - 1] } else { &prev };
            let mut bad = false;

            for k in 0..w {
                if k > 0 && t[k] == t[k - 1] {
                    continue;
                }
                if k + 1 < w && t[k] == t[k + 1] {
                    continue;
                }
                if s[k] == t[k] {
                    continue;
                }

                bad = true;
                break;
            }

            if bad {
                continue;
            }

            res = std::cmp::min(res, dp[h - 1][i][j]);
        }
    }

    if res == INF {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
