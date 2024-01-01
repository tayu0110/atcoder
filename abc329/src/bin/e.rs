use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {n: usize, m: usize, mut s: marker::Bytes, t: marker::Bytes}

    let mut index = (0..n).take_while(|i| i + m <= n).collect::<BTreeSet<_>>();
    let mut updated = true;
    let mut cnt = 0;
    while updated {
        updated = false;
        let mut buf = vec![];
        if cnt % 2 == 0 {
            for &i in &index {
                if &s[i..i + m] == &vec![0; m] {
                    continue;
                }
                let mut w = vec![false; m];
                for j in 0..m {
                    w[j] = s[i + j] == t[j] || s[i + j] == 0;
                }

                if w.into_iter().all(|f| f) {
                    s[i..i + m].fill(0);
                    updated = true;
                    buf.push(i);
                }
            }
        } else {
            for &i in index.iter().rev() {
                if &s[i..i + m] == &vec![0; m] {
                    continue;
                }
                let mut w = vec![false; m];
                for j in 0..m {
                    w[j] = s[i + j] == t[j] || s[i + j] == 0;
                }

                if w.into_iter().all(|f| f) {
                    s[i..i + m].fill(0);
                    updated = true;
                    buf.push(i);
                }
            }
        }

        if s.iter().all(|&c| c == 0) {
            break;
        }

        for b in buf {
            index.remove(&b);
        }

        cnt += 1;
    }

    if s.iter().all(|&c| c == 0) {
        println!("Yes")
    } else {
        println!("No")
    }
}
