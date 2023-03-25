use proconio::input;

fn main() {
    const MOD: i64 = 998244353;
    input! {n: usize, a: [i64; n]}

    let mut dp = vec![0; n+1];
    dp[0] = 1;

    let mut max = vec![];
    let mut min = vec![];
    let mut max_v = vec![];
    let mut min_v = vec![];
    let (mut max_sum, mut min_sum) = (0, 0);
    for i in 0..n {
        let mut sum = dp[i];
        while !max.is_empty() && max.last().unwrap() < &a[i] {
            max_sum -= *max_v.last().unwrap() * max.pop().unwrap();
            max_sum %= MOD;
            sum += max_v.pop().unwrap();
            sum %= MOD;
        }
        max_sum += sum * a[i];
        max_sum %= MOD;
        max.push(a[i]);
        max_v.push(sum);

        let mut sum = dp[i];
        while !min.is_empty() && min.last().unwrap() > &a[i] {
            min_sum -= *min_v.last().unwrap() * min.pop().unwrap();
            min_sum %= MOD;
            sum += min_v.pop().unwrap();
            sum %= MOD;
        }
        min_sum += sum * a[i];
        min_sum %= MOD;
        min.push(a[i]);
        min_v.push(sum);

        dp[i+1] = max_sum - min_sum;
        dp[i+1] %= MOD;
    }

    if dp[n] < 0 {
        dp[n] += MOD;
    }

    println!("{}", dp[n]);
}