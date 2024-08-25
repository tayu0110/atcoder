use cpio::{putln, scan};

fn main() {
    scan!(
        m: usize,
        n: usize,
        k: usize,
        s: [String; m],
        query: [(u16, u16, u16, u16); k]
    );

    let mut map = vec![vec![0u64; n + 1]; m + 1];
    let mut cnt = [0; 3];

    for (i, s) in s.into_iter().enumerate() {
        for (j, s) in s.bytes().enumerate() {
            if s == b'J' {
                cnt[0] += 1;
                map[i + 1][j + 1] = 1;
            } else if s == b'O' {
                cnt[1] += 1;
                map[i + 1][j + 1] = 1000_000;
            } else {
                cnt[2] += 1;
                map[i + 1][j + 1] = 1000_000_000_000;
            }
        }
    }

    if cnt.iter().filter(|&&c| c == 0).count() == 2 {
        let p = cnt.iter().position(|&c| c != 0).unwrap();
        for (u, l, d, r) in query {
            cnt[p] = (r + 1 - l) as u32 * (d + 1 - u) as u32;
            putln!(cnt[0], cnt[1], cnt[2], @sep = " ");
        }
        return;
    }

    for i in 0..=m {
        for j in 0..n {
            map[i][j + 1] += map[i][j];
        }
    }
    for j in 0..=n {
        for i in 0..m {
            map[i + 1][j] += map[i][j];
        }
    }

    for (u, l, d, r) in query {
        let mut i = map[d as usize][r as usize] + map[u as usize - 1][l as usize - 1]
            - map[d as usize][l as usize - 1]
            - map[u as usize - 1][r as usize];

        let j = (i % 1000_000) as u32;
        i /= 1000_000;
        let o = (i % 1000_000) as u32;
        i /= 1000_000;
        putln!(j, o, i as u32, @sep = " ");
    }
}
