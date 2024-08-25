use proconio::input;
use proconio::marker::Chars;

const MOD: i32 = 1000000007;

fn main() {
    input! {h: usize, w: usize, c: [Chars; h]};

    let mut mask = vec![];
    for i in 0..w {
        if i == 0 {
            mask.push(0b110);
        } else if i == w-1 {
            mask.push((1 << w) | 0b11);
        } else {
            mask.push((1 << w) | 0b111);
        }
    }

    let mut buf = vec![vec![]; w];
    for i in 0..(1 << (w+1)) {
        let mut bad = 0;
        let mut pos = 0;
        for j in 0..w {
            if (0b11 << j & i) == 0b11 << j {
                bad += 1;
                pos = j;
            }
        }

        if bad <= 1 {
            for j in 0..w {
                let mut flag = false;
                if bad == 1 && j != w - pos - 1 {
                    flag = true;
                }

                if !flag {
                    buf[(j + 1) % w].push(i);
                }
            }
        }
    }

    let mut dp = vec![0; 1 << (w+1)];
    dp[0] = 1;
    for i in 0..h {
        for j in 0..w {
            let mut tmp = vec![0; 1 << (w+1)];
            for k in &buf[j] {
                let k = *k;
            // for k in 0..(1 << w+1) {
                tmp[k >> 1] += dp[k];
                tmp[k >> 1] %= MOD;
                if (k & mask[j]) != 0 {
                    continue;
                }
                if c[i][j] == '.' {
                    tmp[(1 << w) | (k >> 1)] += dp[k];
                    tmp[(1 << w) | (k >> 1)] %= MOD;
                }
            }

            std::mem::swap(&mut dp, &mut tmp);
        }
    }

    println!("{}", dp.into_iter().fold(0, |sum, x| (sum + x) % MOD));
}