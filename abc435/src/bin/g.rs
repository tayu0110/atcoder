use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut color = vec![vec![0usize; n + 1]; 4];
    for &(l, r) in &e {
        if r - l == 0 {
            continue;
        }
        color[0][l - 1] = color[0][l - 1].wrapping_add(1);
        color[0][r - 1] = color[0][r - 1].wrapping_sub(1);

        if l + 4 <= r {
            color[1][l - 1] = color[1][l - 1].wrapping_add(1);
            color[1][r - 3] = color[1][r - 3].wrapping_sub(1);

            color[2][l + 1] = color[2][l + 1].wrapping_add(1);
            color[2][r - 1] = color[2][r - 1].wrapping_sub(1);
        }
        if l + 6 <= r {
            color[3][l - 1] = color[3][l - 1].wrapping_add(1);
            color[3][r - 1] = color[3][r - 1].wrapping_sub(1);
        }
    }

    for i in 0..n {
        for j in 0..3 {
            color[j][i + 1] = color[j][i + 1].wrapping_add(color[j][i]);
        }
    }

    eprintln!("color: {color:?}");

    let mut dp = vec![vec![0usize; n + 3]; 3];
    dp[0][0] = 1;
    for i in 0..n {
        dp[0][i + 1] += dp[0][i] + dp[1][i] + dp[2][i];
        dp[0][i + 1] %= M;

        if i + 2 <= n {
            dp[1][i + 2] += dp[0][i] * (color[0][i] - color[1][i]) % M;
            dp[1][i + 2] %= M;
            dp[1][i + 2] += dp[1][i] * (color[0][i] - color[1][i]) % M;
            dp[1][i + 2] %= M;
            dp[1][i + 2] += dp[2][i] * (color[0][i] - color[2][i]) % M;
            dp[1][i + 2] %= M;

            dp[2][i + 2] += dp[0][i] * color[1][i] % M;
            dp[2][i + 2] %= M;
            dp[2][i + 2] += dp[1][i] * color[1][i] % M;
            dp[2][i + 2] %= M;
            dp[2][i + 2] += dp[2][i] * (color[1][i] - color[3][i]) % M;
            dp[2][i + 2] %= M;
        }
    }

    println!("{}", dp.iter().map(|dp| dp[n - 1]).sum::<usize>() % M)
}
