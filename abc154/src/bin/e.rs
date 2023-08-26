use proconio::*;

fn main() {
    input! {n: marker::Bytes, k: usize}

    if n.len() < 4 {
        let n = n
            .into_iter()
            .fold(0usize, |s, v| s * 10 + (v - b'0') as usize);
        let mut res = 0;
        for mut i in 1..=n {
            let mut cnt = 0;
            while i > 0 {
                if i % 10 != 0 {
                    cnt += 1;
                }
                i /= 10;
            }

            if cnt == k {
                res += 1;
            }
        }

        println!("{}", res);
        return;
    }

    let len = n.len();
    let less = if k == 1 {
        9 * (len - 1)
    } else if k == 2 {
        9 * 9 * (len - 1) * (len - 2) / 2
    } else {
        9 * 9 * 9 * (len - 1) * (len - 2) * (len - 3) / 6
    };

    let mut dp = vec![vec![vec![vec![0; 2]; k]; 10]; len];
    dp[0][(n[0] - b'0') as usize][0][1] = 1;
    for i in 1..(n[0] - b'0') as usize {
        dp[0][i][0][0] = 1;
    }
    for i in 1..len {
        let c = (n[i] - b'0') as usize;
        for prev in 0..10 {
            for next in 0..10 {
                for j in 0..k {
                    if j == k - 1 && next != 0 {
                        continue;
                    }
                    for same in 0..2 {
                        if same == 1 && next > c {
                            continue;
                        }
                        if same == 1 && next == c {
                            dp[i][next][j + (next != 0) as usize][1] += dp[i - 1][prev][j][1];
                        } else {
                            dp[i][next][j + (next != 0) as usize][0] += dp[i - 1][prev][j][same];
                        }
                    }
                }
            }
        }
    }

    let mut res = less;
    for i in 0..10 {
        for j in 0..2 {
            res += dp[len - 1][i][k - 1][j];
        }
    }
    println!("{}", res)
}
