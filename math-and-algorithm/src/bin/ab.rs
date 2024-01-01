use proconio::*;

const N: usize = 45;
const RES: [usize; N + 1] = {
    let mut dp = [0; N + 1];
    dp[0] = 1;
    let mut i = 0;
    while i < N {
        dp[i + 1] += dp[i];
        if i + 2 <= N {
            dp[i + 2] += dp[i];
        }
        i += 1;
    }
    dp
};

fn main() {
    input! {n: u32}
    println!("{}", RES[n as usize])
}
