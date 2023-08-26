use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut dp = vec![vec![0; 1001]; 6];
    dp[0][0] = 1;
    for a in a {
        for i in (0..5).rev() {
            for j in (0..1000).rev() {
                if j + a <= 1000 {
                    dp[i + 1][j + a] += dp[i][j];
                }
            }
        }
    }

    println!("{}", dp[5][1000])
}
