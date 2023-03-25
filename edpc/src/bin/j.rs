use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    const MAX: usize = 300;

    let v = a.iter().fold([0; 3], |mut s, v| {
        s[*v - 1] += 1;
        s
    });

    let mut dp = vec![vec![vec![0.0; MAX + 1]; MAX + 1]; MAX + 1];
    for k in 0..=MAX {
        for j in 0..=MAX {
            for i in 0..=MAX {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }

                if i + j + k > n {
                    continue;
                }

                dp[i][j][k] += n as f64;
                if i > 0 {
                    dp[i][j][k] += dp[i - 1][j][k] * i as f64;
                }
                if j > 0 {
                    dp[i][j][k] += dp[i + 1][j - 1][k] * j as f64;
                }
                if k > 0 {
                    dp[i][j][k] += dp[i][j + 1][k - 1] * k as f64;
                }

                dp[i][j][k] /= (i + j + k) as f64;
            }
        }
    }

    println!("{}", dp[v[0]][v[1]][v[2]])
}
