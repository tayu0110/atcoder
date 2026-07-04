use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let utpc = b"UTPC";
    let mut res = usize::MAX;
    for (i, t) in s.windows(4).enumerate() {
        let mut cnt = 0;
        let mut s = s.clone();
        for j in i..i + 4 {
            if utpc[j - i] != s[j] {
                cnt += 1;
                for k in i..i + 4 {
                    if s[k] == utpc[j - i] {
                        s.swap(k, j - i);
                        break;
                    }
                }
                if utpc[j - i] != s[j] {
                    for k in 0..n {
                        if s[k] == utpc[j - i] {
                            s.swap(k, j - i);
                            break;
                        }
                    }
                }
            }
        }

        res = res.min(cnt);
    }
}
