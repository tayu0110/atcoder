use proconio::*;

fn main() {
    input! {n: usize, c: usize, p: [(usize, usize, usize); n]}

    let mut backet = vec![vec![]; c];
    for (s, t, c) in p {
        backet[c - 1].push((s, t));
    }

    let mut buf = vec![];
    for v in backet.iter_mut() {
        if v.is_empty() {
            continue;
        }

        v.sort();
        v.reverse();

        let mut prev = v.pop().unwrap();
        while let Some((s, t)) = v.pop() {
            if prev.1 == s {
                prev.1 = t;
            } else {
                buf.push((prev.1, prev.0));
                prev = (s, t);
            }
        }

        buf.push((prev.1, prev.0));
    }

    buf.sort();

    let mut v = vec![];
    for (t, s) in buf.drain(..) {
        let mut found = false;
        for nv in v.iter_mut().rev() {
            if *nv < s {
                *nv = t;
                found = true;
                break;
            }
        }

        if !found {
            v.push(t);
        }

        v.sort();
    }

    println!("{}", v.len());
}
