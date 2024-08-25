use proconio::input;

fn main() {
    input! {n: usize, mut h: [usize; n], mut p: [(u16, u32, u32); n]}

    p.sort_by_key(|&(_, s, e)| (e, s));

    let mut c = vec![false; n];
    for &(m, _, _) in &p {
        c[m as usize - 1] = true;
    }
    let c = c
        .into_iter()
        .enumerate()
        .filter(|&(_, t)| t)
        .map(|(i, _)| i as u16)
        .collect::<Vec<_>>();

    let n = c.len();
    let mut dp = vec![vec![]; n + 1];
    dp[n].push((0usize, 0u32));

    for (m, s, e) in p {
        let m = c.iter().position(|i| *i == m - 1).unwrap();

        for i in (0..dp[m].len()).rev() {
            if dp[m][i].1 <= s {
                if i + 1 == dp[m].len() {
                    let tmp = dp[m][i].0;
                    dp[m].push((tmp + h[i + 1], e));
                } else if dp[m][i + 1].0 < dp[m][i].0 + h[i + 1] {
                    dp[m][i + 1] = (dp[m][i].0 + h[i + 1], e);
                }
            }
        }

        for i in 0..=n {
            if i == m {
                continue;
            }
            for j in 0..dp[i].len() {
                if dp[i][j].1 <= s {
                    if dp[m].is_empty() {
                        let tmp = dp[i][j].0;
                        dp[m].push((tmp + h[0], e));
                    } else if dp[m][0].0 < dp[i][j].0 + h[0] {
                        dp[m][0] = (dp[i][j].0 + h[0], e);
                    }
                }
            }
        }
    }

    let mut res = 0;
    for v in dp {
        res = std::cmp::max(res, v.into_iter().map(|(res, _)| res).max().unwrap());
    }

    println!("{}", res);
}
