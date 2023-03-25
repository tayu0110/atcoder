use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], c: [usize; n], x: [usize; m]}

    let set = x
        .into_iter()
        .map(|v| v - 1)
        .collect::<std::collections::HashSet<_>>();

    let mut cost = vec![vec![std::usize::MAX; n + 1]; n];
    for i in 0..n {
        cost[i][1] = c[i];
        for j in 1..=i {
            cost[i][j + 1] = std::cmp::min(cost[i][j], c[i - j]);
        }
    }

    let mut dp = vec![std::usize::MAX; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut new = vec![std::usize::MAX; n + 1];

        for j in 0..=i {
            if dp[j] < std::usize::MAX {
                new[j + 1] = std::cmp::min(new[j + 1], dp[j] + a[i] + cost[i][j + 1]);
                if !set.contains(&i) {
                    new[j] = std::cmp::min(new[j], dp[j]);
                }
            }
        }
        dp = new;
    }

    println!("{}", dp[m..].into_iter().min().unwrap())
}
