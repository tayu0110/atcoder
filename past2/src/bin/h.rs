use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [marker::Chars; n]}

    let mut t = vec![vec![]; 11];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 'S' {
                t[0].push((i, j));
            } else if a[i][j] == 'G' {
                t[10].push((i, j));
            } else {
                t[a[i][j] as usize - b'0' as usize].push((i, j));
            }
        }
    }

    let mut dp = vec![];
    for t in &t {
        dp.push(vec![usize::MAX; t.len()]);
    }
    dp[0][0] = 0;
    for i in 0..10 {
        let plen = dp[i].len();
        let nlen = dp[i + 1].len();
        for j in 0..plen {
            if dp[i][j] == std::usize::MAX {
                continue;
            }
            let (r, c) = t[i][j];
            for k in 0..nlen {
                let (nr, nc) = t[i + 1][k];

                dp[i + 1][k] =
                    dp[i + 1][k].min(dp[i][j] + nr.max(r) - nr.min(r) + nc.max(c) - nc.min(c));
            }
        }
    }

    println!("{}", dp[10][0] as i64)
}
