fn main() {
    proconio::input! {s: proconio::marker::Chars, k: usize}

    let mut p = s.iter().enumerate().filter(|&(_, &c)| c == 'Y').map(|(i, _)| i).collect::<Vec<_>>();
    p.iter_mut().enumerate().for_each(|(i, j)| *j -= i);

    let cnt = p.len();
    let mut sum = vec![0; cnt + 1];
    for i in 0..cnt {
        sum[i+1] = sum[i] + p[i];
    }

    let mut dp = vec![0; cnt+1];
    for i in 0..=cnt {
        if i > 0 {
            dp[i] = dp[i-1];
        }

        while dp[i] < cnt {
            let (l, r) = (i, dp[i]+1);
            let m = (r + l) / 2;
            let s1 = (m - l) * p[m] - (sum[m] - sum[l]);
            let s2 = (sum[r] - sum[m]) - (r - m) * p[m];

            if s1 + s2 <= k {
                dp[i] += 1;
            } else {
                break;
            }
        }
    }

    let res = dp.into_iter().enumerate().fold(0, |s, (i, v)| std::cmp::max(s, v - i));
    println!("{}", res);
}