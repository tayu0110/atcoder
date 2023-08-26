use proconio::input;

fn main() {
    input! {n: usize, s: usize, a: [usize; n]}

    let mut dp = vec![false; s + 1];
    dp[0] = true;
    for a in a {
        for i in (0..s).rev() {
            if dp[i] && i + a <= s {
                dp[i + a] = true;
            }
        }
    }

    if dp[s] {
        println!("Yes")
    } else {
        println!("No")
    }
}
