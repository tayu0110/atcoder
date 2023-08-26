use proconio::*;

fn solve(s: &[char]) -> usize {
    let mut res = 0;
    let mut prev = 0;
    let mut cnt = 0;
    for &c in s {
        if c == '2' {
            if prev == 2 {
                res = res.max(cnt / 2 * 2);
                cnt = 1;
                prev = 2;
            } else {
                cnt += 1;
                prev = 2;
            }
        } else if c == '5' {
            if prev != 2 {
                res = res.max(cnt / 2 * 2);
                cnt = 0;
                prev = 0;
            } else {
                cnt += 1;
                prev = 5;
            }
        } else {
            res = res.max(cnt / 2 * 2);
            cnt = 0;
            prev = 0;
        }
    }
    res.max(cnt / 2 * 2)
}

fn main() {
    input! {t: marker::Chars}

    let mut res = 0;
    for i in 0..2 {
        let mut s = t.clone();
        for j in 0..t.len() {
            if s[j] == '?' {
                if i == j % 2 {
                    s[j] = '2';
                } else {
                    s[j] = '5';
                }
            }
        }
        res = res.max(solve(&s));
    }
    println!("{}", res)
}
