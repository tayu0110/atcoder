use proconio::*;

fn main() {
    input! {a: [[usize; 5]; 5]}

    let mut dp = vec![0u32; 1 << 25];
    let mut pos = vec![usize::MAX; 26];
    for i in 0..5 {
        for j in 0..5 {
            if a[i][j] != 0 {
                pos[a[i][j]] = i * 5 + j;
            }
        }
    }

    dp[0] = 1;
    for i in 1usize..1 << 25 {
        let now = i.count_ones() as usize;
        for j in 0..25 {
            if i & (1 << j) != 0 {
                if pos[now] < usize::MAX && j != pos[now] {
                    continue;
                }

                let prev = i - (1 << j);
                let (r, c) = (j / 5, j % 5);
                if (1..=3).contains(&r) {
                    let up = (r - 1) * 5 + c;
                    let down = (r + 1) * 5 + c;
                    if (prev & (1 << up) != 0) != (prev & (1 << down) != 0) {
                        continue;
                    }
                }

                if (1..=3).contains(&c) {
                    let left = r * 5 + (c - 1);
                    let right = r * 5 + (c + 1);
                    if (prev & (1 << left) != 0) != (prev & (1 << right) != 0) {
                        continue;
                    }
                }

                dp[i] += dp[prev];
                dp[i] %= 1000000007;
            }
        }
    }

    println!("{}", dp[(1 << 25) - 1]);
}
