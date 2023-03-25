use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], q: usize, q: [(usize, usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut dp = vec![vec![0; n]; 11];
    for (i, &(v, d, _)) in q.iter().enumerate() {
        dp[d][v - 1] = std::cmp::max(dp[d][v - 1], i + 1);
    }

    for i in (1..=10).rev() {
        for j in 0..n {
            dp[i - 1][j] = std::cmp::max(dp[i - 1][j], dp[i][j]);
            for &to in &t[j] {
                dp[i - 1][to] = std::cmp::max(dp[i - 1][to], dp[i][j]);
            }
        }
    }

    for i in 0..n {
        let res = dp[0][i];

        if res == 0 {
            println!("0")
        } else {
            let (_, _, c) = q[res - 1];

            println!("{}", c)
        }
    }
}
