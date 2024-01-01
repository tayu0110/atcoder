use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, t: marker::Chars, s: [marker::Chars; n]}

    let mut res = vec![];
    for (i, s) in s.into_iter().enumerate() {
        if s.len() == t.len() {
            if s.iter().zip(t.iter()).filter(|(s, t)| s != t).count() < 2 {
                res.push(i);
            }
        } else if s.len().abs_diff(t.len()) == 1 {
            let (s, t) = if s.len() < t.len() {
                (&s, &t)
            } else {
                (&t, &s)
            };
            let (mut si, mut ti, mut skip) = (0, 0, 0);
            while si < s.len() && ti < t.len() {
                if s[si] == t[ti] {
                    si += 1;
                } else {
                    skip += 1;
                }
                ti += 1;
            }

            if skip <= 1 {
                res.push(i);
            }
        }
    }

    println!("{}", res.len());
    println!("{}", res.iter().map(|res| res + 1).join(" "))
}
