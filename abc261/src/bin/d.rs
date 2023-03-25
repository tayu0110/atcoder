#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, m: usize, x: [i64; n], p: [(i64, i64); m]};

    let mut map = std::collections::HashMap::new();
    for (c, y) in p {
        *map.entry(c).or_insert(0i64) += y;
    }

    let mut dp = vec![vec![-1i64; n+1]; n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] < 0 {
                continue;
            }
            let now = dp[i][j];
            let mut omote = now + x[i];
            if map.contains_key(&(j as i64 +1)) {
                omote += map.get(&(j as i64+1)).unwrap();
            }
            let ura = now;
            dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], omote);
            dp[i+1][0] = std::cmp::max(dp[i+1][0], ura);
        }
    }
    // eprintln!("{:?}", dp);
    println!("{}", dp[n].iter().max().unwrap());
}
