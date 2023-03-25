use proconio::input;

const MOD: usize = 1000_000_007;

fn main() {
    input! {n: usize, a: [[u8; n]; n]}

    let mut dp = vec![vec![0; 1 << n]; n];
    let mut mask = 0;
    for i in 0..n {
        if a[0][i] == 1 {
            dp[0][1 << i] = 1;
            mask |= 1 << i;
        }
    }

    for i in 0..n - 1 {
        let mut m = 0;
        for k in 0..n {
            if a[i + 1][k] == 0 {
                continue;
            }

            let mut j = mask & !(1 << k);
            let mask = j;
            while j > 0 {
                if dp[i][j] != 0 {
                    dp[i + 1][j | (1 << k)] += dp[i][j];
                    dp[i + 1][j | (1 << k)] %= MOD;
                }

                j = (j - 1) & mask;
            }

            m |= 1 << k;
        }

        mask |= m;
    }

    println!("{}", dp[n - 1][(1 << n) - 1])
}
