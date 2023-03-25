#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! { n: Chars, m: usize, c: [usize; m] }

    let set = c.iter().fold(0, |s, v| s | (1 << *v));

    let mut sum = vec![vec![vec![0usize; 1 << 10]; 2]; n.len()+1];
    let mut dp = vec![vec![vec![0usize; 1 << 10]; 2]; n.len()+1];

    for i in 1..(n[0] as u8 - b'0') as usize {
        dp[1][1][1 << i] = 1;
        sum[1][1][1 << i] = i;
    }

    dp[1][0][1 << (n[0] as u8 - b'0')] = 1;
    sum[1][0][1 << (n[0] as u8 - b'0')] = n[0].to_digit(10).unwrap() as usize;

    for i in 1..n.len() {
        for j in 1..10 {
            dp[i+1][1][1 << j] = 1;
            sum[i+1][1][1 << j] = j;
        }
    }

    for i in 1..n.len() {
        let nd = n[i].to_digit(10).unwrap();
        for j in 0..2 {
            for k in 0..(1 << 10) {
                for l in 0..10 {
                    if j == 0 && l > nd {
                        break;
                    }
                    let mut nj = j;
                    if l < nd {
                        nj = 1;
                    }
                    let nk = k | (1 << l);
                    dp[i+1][nj][nk] += dp[i][j][k];
                    dp[i+1][nj][nk] %= 998244353;
                    sum[i+1][nj][nk] += sum[i][j][k] * 10 + dp[i][j][k] * l as usize;
                    sum[i+1][nj][nk] %= 998244353;
                }
            }
        }
    }

    let res = (0..1 << 10).filter(|v| (*v & set) == set).fold(0, |s, v| (s + sum[n.len()][0][v] + sum[n.len()][1][v]) % 998244353);
    println!("{}", res);
}
