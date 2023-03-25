use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let n = s.len();

    let mut t = vec![vec![]; 26];
    for (i, c) in s.iter().enumerate() {
        t[*c as usize - b'a' as usize].push(i);
    }
    for v in t.iter_mut() {
        v.push(n);
    }

    let mut best = vec![0; n + 2];
    let mut bestl = vec![None; n + 2];
    for i in (0..=n).rev() {
        best[i] = std::usize::MAX >> 10;
        for c in 0..26 {
            let (mut l, mut r) = (-1, t[c].len() as i32);
            while r - l > 1 {
                let m = (r + l) / 2;
                if t[c][m as usize] < i {
                    l = m;
                } else {
                    r = m;
                }
            }

            let next = t[c][r as usize];
            if best[next + 1] + 1 < best[i] {
                bestl[i] = Some(c);
                best[i] = best[next + 1] + 1;
            }
        }
    }

    let mut r = 0;
    let mut res = String::new();
    while r < n {
        if bestl[r].is_none() {
            r += 1;
        } else {
            let c = bestl[r].unwrap();
            res.push((c as u8 + b'a') as char);
            let (mut a, mut b) = (-1, t[c].len() as i32);
            while b - a > 1 {
                let m = (b + a) / 2;
                if t[c][m as usize] < r {
                    a = m;
                } else {
                    b = m;
                }
            }

            r = t[c][b as usize] + 1;
        }
    }

    println!("{}", res)
}
