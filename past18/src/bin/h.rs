use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Bytes; n]}

    let mut next = vec![vec![n as u16; n]; n];
    for i in 0..n {
        let mut now = n;
        for j in (0..n).rev() {
            if s[i][j] == b'.' {
                now = j;
            }
            next[i][j] = now as u16;
        }
    }
    let mut cum = vec![vec![0u16; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            cum[i][j + 1] = cum[i][j] + (s[i][j] == b'#') as u16;
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            let len = next[i][j] as usize - j;
            if len >= 3 {
                for k in (i + 1..(i + len).min(n)).take_while(|&k| s[k][j] == b'#') {
                    let l = next[k][j] as usize - j;
                    if l >= k - i + 1 {
                        let l = k - i + 1;
                        if (i + 1..k).all(|k| cum[k][j + l] - cum[k][j] == 1) {
                            res = res.max(k - i + 1);
                        }
                        break;
                    } else if s[i][j] == b'.' {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", res.saturating_sub(2))
}
