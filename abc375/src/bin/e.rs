use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n]}

    let sum = e.iter().map(|e| e.1).sum::<usize>();
    if sum % 3 != 0 {
        println!("-1");
        return;
    }

    let m = sum / 3;
    let mut dp = vec![vec![usize::MAX; m + 1]; m + 1];
    dp[0][0] = 0;
    for (a, b) in e {
        let mut new = vec![vec![usize::MAX; m + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=m {
                if dp[i][j] == usize::MAX {
                    continue;
                }

                // team 1
                if i + b <= m {
                    new[i + b][j] = new[i + b][j].min(dp[i][j] + (a != 1) as usize);
                }

                // team 2
                if j + b <= m {
                    new[i][j + b] = new[i][j + b].min(dp[i][j] + (a != 2) as usize);
                }

                // team 3
                new[i][j] = new[i][j].min(dp[i][j] + (a != 3) as usize);
            }
        }
        dp = new;
    }

    println!("{}", dp[m][m] as i64);
}
