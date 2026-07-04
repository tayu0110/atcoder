use proconio::*;

fn solve(r: usize) -> usize {
    let rs = r.to_string().bytes().map(|b| b - b'0').collect::<Vec<_>>();
    let mut res = 0;
    for len in 1..rs.len() {
        for top in 1usize..10 {
            res += top.pow(len as u32 - 1);
        }
    }
    let mut dp = [[[0; 2]; 10]; 30];
    for i in 1..rs[0] {
        dp[0][i as usize][0] = 1;
    }
    dp[0][rs[0] as usize][1] = 1;
    for i in 0..rs.len() - 1 {
        for j in 0..10 {
            for k in 0..2 {
                if dp[i][j][k] == 0 {
                    continue;
                }

                for l in 0..j {
                    if k == 1 && l as u8 > rs[i + 1] {
                        break;
                    }

                    let nk = (k == 1 && l as u8 == rs[i + 1]) as usize;
                    dp[i + 1][j][nk] += dp[i][j][k]
                }
            }
        }
    }
    for i in 0..10 {
        for j in 0..2 {
            res += dp[rs.len() - 1][i][j];
        }
    }
    res
}

fn main() {
    input! {l: usize, r: usize}
    println!("{}", solve(r) - solve(l - 1))
}
