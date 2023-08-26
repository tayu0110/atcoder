use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn ord(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - b'A' as usize
    } else {
        c as usize - b'a' as usize + 26
    }
}

fn rec(
    now: usize,
    cond: usize,
    up: &mut Vec<char>,
    s: &[char],
    memo: &mut Vec<Vec<Vec<Option<Modint>>>>,
) -> Modint {
    if now == s.len() {
        return Modint::one();
    }

    if cond == 0 {
        if s[now] != '?' {
            if let Some(res) = memo[cond][now][ord(s[now])] {
                return res;
            }

            if s[now].is_uppercase() {
                let res = if up.contains(&s[now]) {
                    rec(now + 1, cond + 1, up, s, memo)
                } else {
                    up.push(s[now]);
                    let res = rec(now + 1, cond, up, s, memo);
                    up.pop().unwrap();
                    res
                };

                memo[cond][now][ord(s[now])] = Some(res);
                return res;
            }

            let res = rec(now + 1, cond, up, s, memo);
            memo[cond][now][ord(s[now])] = Some(res);
            return res;
        }

        let mut res = Modint::zero();
        for c in 'A'..='Z' {
            if let Some(r) = memo[cond][now][ord(c)] {
                res += r;
                continue;
            }
            let r = if up.contains(&c) {
                rec(now + 1, cond + 1, up, s, memo)
            } else {
                up.push(c);
                let res = rec(now + 1, cond, up, s, memo);
                up.pop().unwrap();
                res
            };

            memo[cond][now][ord(c)] = Some(r);
            res += r;
        }

        for c in 'a'..='z' {
            if let Some(r) = memo[cond][now][ord(c)] {
                res += r;
                continue;
            }
            let r = rec(now + 1, cond, up, s, memo);
            memo[cond][now][ord(c)] = Some(r);
            res += r;
        }

        res
    } else if cond == 1 {
        if s[now] != '?' {
            if let Some(res) = memo[cond][now][ord(s[now])] {
                return res;
            }

            if s[now].is_lowercase() {
                let res = rec(now + 1, cond + 1, up, s, memo);
                memo[cond][now][ord(s[now])] = Some(res);
                return res;
            }

            let res = rec(now + 1, cond, up, s, memo);
            memo[cond][now][ord(s[now])] = Some(res);
            return res;
        }

        let mut res = Modint::zero();
        for c in 'A'..='Z' {
            if let Some(r) = memo[cond][now][ord(c)] {
                res += r;
                continue;
            }
            let r = rec(now + 1, cond, up, s, memo);
            memo[cond][now][ord(c)] = Some(r);
            res += r;
        }

        for c in 'a'..='z' {
            if let Some(r) = memo[cond][now][ord(c)] {
                res += r;
                continue;
            }
            let r = rec(now + 1, cond + 1, up, s, memo);
            memo[cond][now][ord(c)] = Some(r);
            res += r;
        }

        res
    } else {
        if s[now] != '?' && s[now].is_uppercase() {
            memo[cond][now][ord(s[now])] = Some(Modint::zero());
            return Modint::zero();
        }

        if s[now] != '?' {
            if let Some(r) = memo[cond][now][ord(s[now])] {
                return r;
            }

            let res = rec(now + 1, cond, up, s, memo);
            memo[cond][now][ord(s[now])] = Some(res);
            return res;
        }

        let mut res = Modint::zero();
        for c in 'a'..='z' {
            if let Some(r) = memo[cond][now][ord(c)] {
                res += r;
            } else {
                let r = rec(now + 1, cond, up, s, memo);
                memo[cond][now][ord(c)] = Some(r);
                res += r;
            }
        }

        res
    }
}

fn main() {
    input! {s: marker::Chars}
    let n = s.len();

    let mut memo = vec![vec![vec![None; 52]; n]; 3];
    let res = rec(0, 0, &mut vec![], &s, &mut memo);
    eprintln!("{:?}", memo);
    println!("{}", res);
}
