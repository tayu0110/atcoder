use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    const INDEX: [u8; 128] = {
        let mut b = [0; 128];
        b[b'J' as usize] = 0;
        b[b'O' as usize] = 1;
        b[b'I' as usize] = 2;
        b
    };

    let mut dp = [0; 1 << 3];
    dp[1] = 1;
    let mut next = [0; 1 << 3];
    for j in s.into_iter().map(|s| INDEX[s as usize]) {
        for i in 0..1 << 3 {
            for ni in 0..1 << 3 {
                if i & ni != 0 && ni & (1 << j) != 0 {
                    next[ni] += dp[i];
                    next[ni] %= 10007;
                }
            }
        }

        (dp, next) = (next, dp);
        next.fill(0);
    }

    println!("{}", dp.iter().fold(0, |s, v| (s + v) % 10007));
}
