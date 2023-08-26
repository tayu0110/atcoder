use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut dp = vec![std::usize::MAX; n + 1];
    for a in a {
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if a < dp[m as usize] {
                r = m;
            } else {
                l = m;
            }
        }

        dp[r as usize] = a;
    }

    println!("{}", dp.iter().position(|&v| v == std::usize::MAX).unwrap())
}
