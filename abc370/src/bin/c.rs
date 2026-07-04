use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut s: marker::Chars, t: marker::Chars}

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for (i, (&s, &t)) in s.iter().zip(t.iter()).enumerate() {
        if s != t {
            if s > t {
                a.push((i, t));
            } else if s == t {
                b.push((i, t));
            } else {
                c.push((i, t));
            }
        }
    }
    c.reverse();

    let mut x = vec![];
    for c in [a, b, c] {
        for (pos, to) in c {
            s[pos] = to;
            x.push(s.iter().collect::<String>());
        }
    }

    println!("{}", x.len());
    println!("{}", x.into_iter().join("\n"))
}
