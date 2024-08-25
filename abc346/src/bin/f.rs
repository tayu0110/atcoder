use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Bytes, mut t: marker::Bytes}
    s.iter_mut().for_each(|s| *s -= b'a');
    t.iter_mut().for_each(|t| *t -= b'a');

    let slen = s.len();
    let tlen = t.len();

    let mut memo = vec![vec![0; 26]; slen + 1];
    for (i, &s) in s.iter().enumerate() {
        memo[i + 1][s as usize] += 1;
        for j in 0..26 {
            memo[i + 1][j] += memo[i][j];
        }
    }

    let (mut l, mut r) = (0, (n * slen + tlen - 1) / tlen + 1);
    while r - l > 1 {
        let k = (r + l) / 2;

        let mut next = 0;
        for &t in &t {
            if memo[slen][t as usize] == 0 {
                println!("0");
                return;
            }

            if k == 0 {
                continue;
            }

            let mut nk = k;
            if next % slen != 0 {
                let i = next % slen;
                if memo[slen][t as usize] - memo[i][t as usize] < nk {
                    nk -= memo[slen][t as usize] - memo[i][t as usize];
                    next = (next + slen - 1) / slen * slen;
                } else {
                    let pos =
                        memo[i..].partition_point(|v| v[t as usize] - memo[i][t as usize] < nk);
                    next = next / slen * slen + i + pos;
                    continue;
                }
            }

            let sum = memo[slen][t as usize];
            let d = (nk - 1) / sum;
            next += d * slen;
            nk -= d * sum;
            // eprintln!("t: {t}, next: {next}, nk: {nk}");

            if nk > 0 {
                let pos = memo.partition_point(|v| v[t as usize] < nk);
                // eprintln!("pos: {pos}");
                next += pos;
            }
            // eprintln!("t: {t}, next: {next}");
        }

        // eprintln!("k: {k}, next: {next}");
        if next <= slen * n {
            l = k;
        } else {
            r = k;
        }
    }

    println!("{l}");
}
