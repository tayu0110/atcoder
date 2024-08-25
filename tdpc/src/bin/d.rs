use proconio::*;

fn main() {
    input! {n: usize, mut d: usize}

    let mut div = [0; 6];
    for i in 2..6 {
        let t = i;
        while d % t == 0 {
            div[i] += 1;
            d /= t;
        }
    }

    if d != 1 {
        println!("0");
        return;
    }

    let mut dp = vec![vec![vec![vec![0.0; div[5] + 1]; div[3] + 1]; div[2] + 1]; n + 1];
    dp[n][div[2]][div[3]][div[5]] = 1.0;
    for i in (1..=n).rev() {
        for j in 0..=div[2] {
            for k in 0..=div[3] {
                for l in 0..=div[5] {
                    let now = dp[i][j][k][l];
                    dp[i - 1][j][k][l] += now / 6.0;
                    dp[i - 1][j.saturating_sub(1)][k][l] += now / 6.0;
                    dp[i - 1][j][k.saturating_sub(1)][l] += now / 6.0;
                    dp[i - 1][j.saturating_sub(2)][k][l] += now / 6.0;
                    dp[i - 1][j][k][l.saturating_sub(1)] += now / 6.0;
                    dp[i - 1][j.saturating_sub(1)][k.saturating_sub(1)][l] += now / 6.0;
                }
            }
        }
    }

    println!("{}", dp[0][0][0][0])
}
