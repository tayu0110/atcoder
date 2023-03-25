use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut dp = vec![vec![std::usize::MAX; n + 1]; n];
    for i in 0..n {
        dp[i][i + 1] = 0;
    }

    let mut size = vec![vec![0; n + 1]; n];
    for i in 0..n {
        for j in i + 1..=n {
            size[i][j] = a[i..j].iter().sum::<usize>();
        }
    }

    for len in 2..=n {
        for left in 0..n {
            let right = left + len;
            if right > n {
                break;
            }

            for mid in left + 1..right {
                dp[left][right] = std::cmp::min(
                    dp[left][right],
                    dp[left][mid] + dp[mid][right] + size[left][mid] + size[mid][right],
                );
            }
        }
    }

    println!("{}", dp[0][n]);
}
