use proconio::*;

fn solve(n: usize, a: usize, c: usize, s: &[usize]) -> Vec<[usize; 4]> {
    let mut dp = vec![[usize::MAX; 4]; n + 1];
    dp[0][3] = 0;

    for (i, &s) in s.iter().enumerate() {
        for j in 0..4 {
            if dp[i][j] == usize::MAX {
                continue;
            }

            if j < 3 {
                let next = (s + 1) % 3;
                if j == next {
                    dp[i + 1][next] = dp[i + 1][next].min(dp[i][j]);
                } else {
                    dp[i + 1][next] = dp[i + 1][next].min(dp[i][j] + c);
                }
            } else {
                dp[i + 1][0] = dp[i + 1][0].min(dp[i][j] + c);
                dp[i + 1][3] = dp[i + 1][3].min(dp[i][j] + a);
            }

            if j == 2 {
                dp[i + 1][3] = dp[i + 1][3].min(dp[i][j] + a);
            }
        }
    }

    dp
}

fn main() {
    input! {n: usize, s: marker::Bytes, a: usize, b: usize, c: usize}

    let s = s
        .into_iter()
        .map(|c| {
            if c == b'R' {
                0
            } else if c == b'G' {
                1
            } else {
                2
            }
        })
        .collect::<Vec<_>>();
    let r1 = solve(n, a, c, &s);
    let mut r2 = solve(
        n,
        b,
        c,
        &s.into_iter()
            .rev()
            .map(|c| {
                if c == 0 {
                    2
                } else if c == 2 {
                    0
                } else {
                    c
                }
            })
            .collect::<Vec<_>>(),
    );

    r2.reverse();

    let mut res = usize::MAX;
    for i in 0..n {
        for j in 0..4 {
            if j == 3 {
                res = res.min(r1[i][j].saturating_add(r2[i][0]));
                res = res.min(r1[i][j].saturating_add(r2[i][3]));
            } else {
                res = res.min(r1[i][j].saturating_add(r2[i][(j + 1) % 3]));
                res = res.min(r1[i][j].saturating_add(r2[i][3]));
            }
        }
    }

    println!("{res}")
}
