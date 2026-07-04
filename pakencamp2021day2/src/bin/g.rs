use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], a: [[i32; n]; n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut dp = vec![-1; 1 << n];
    dp[1] = 0;
    for i in 1..(1 << n) - 1 {
        if dp[i] < 0 {
            continue;
        }
        for j in 0..n {
            if i & (1 << j) != 0 {
                for &to in &t[j] {
                    if i & (1 << to) == 0 {
                        let next = i | (1 << to);
                        dp[next] = dp[next].max(dp[i] + a[i.count_ones() as usize - 1][to]);
                    }
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1])
}
