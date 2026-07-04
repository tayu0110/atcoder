use proconio::*;

fn main() {
    input! {n: usize, m: usize, w: usize, x: usize, y: usize, z: usize, e: [(usize, usize, usize); m]}

    let mut rev = vec![vec![]; n];
    for (c, s, t) in e {
        rev[t - 1].push((s - 1, c));
    }

    // 0: from w
    // 1: from y
    // 2: from both w and y
    let mut dp = vec![vec![usize::MAX; n]; 3];
    dp[0][w - 1] = 0;
    dp[1][y - 1] = 0;
    for i in 0..n {
        for j in 0..3 {
            for &(from, w) in &rev[i] {
                dp[j][i] = dp[j][i].min(dp[j][from].min(dp[2][from]).saturating_add(w));
            }
        }

        dp[2][i] = dp[2][i].min(dp[0][i].saturating_add(dp[1][i]));
    }
    let res = dp[0][x - 1].saturating_add(dp[1][z - 1]);
    if res == usize::MAX {
        println!("Impossible")
    } else {
        println!("{res}")
    }
}
