#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut s: Chars, k: usize}
    
    let mut v = [0usize; 3];
    let mut map = std::collections::HashMap::new();
    map.insert('K', 0usize);
    map.insert('E', 1usize);
    map.insert('Y', 2usize);
    let len = s.len();
    
    let mut pos = vec![vec![]; 3];
    for (i, c) in s.iter().enumerate() {
        pos[*map.get(c).unwrap()].push(i);
        v[*map.get(c).unwrap()] += 1;
    }

    let max = len * (len-1) / 2 + 1;
    let mut dp = vec![vec![vec![vec![0usize; max]; v[2]+1]; v[1]+1]; v[0]+1];
    dp[0][0][0][0] = 1;
    for i in 0..v[0]+1 {
        for j in 0..v[1]+1 {
            for l in 0..v[2]+1 {
                if i < v[0] {
                    let idx = pos[0][i];
                    let mut inv = 0;
                    for b in 0..j {
                        if pos[1][b] > idx {
                            inv += 1;
                        }
                    }
                    for c in 0..l {
                        if pos[2][c] > idx {
                            inv += 1;
                        }
                    }
                    for u in inv..max {
                        dp[i+1][j][l][u] += dp[i][j][l][u-inv];
                    }
                }
                if j < v[1] {
                    let idx = pos[1][j];
                    let mut inv = 0;
                    for a in 0..i {
                        if pos[0][a] > idx {
                            inv += 1;
                        }
                    }
                    for c in 0..l {
                        if pos[2][c] > idx {
                            inv += 1;
                        }
                    }
                    for u in inv..max {
                        dp[i][j+1][l][u] += dp[i][j][l][u-inv];
                    }
                }
                if l < v[2] {
                    let idx = pos[2][l];
                    let mut inv = 0;
                    for a in 0..i {
                        if pos[0][a] > idx {
                            inv += 1;
                        }
                    }
                    for b in 0..j {
                        if pos[1][b] > idx {
                            inv += 1;
                        }
                    }
                    for u in inv..max {
                        dp[i][j][l+1][u] += dp[i][j][l][u-inv];
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..std::cmp::min(max, k+1) {
        res += dp[v[0]][v[1]][v[2]][i];
    }

    println!("{}", res);
}
