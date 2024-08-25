use proconio::*;

#[fastout]
fn main() {
    input! {s: marker::Chars}

    let mut res = vec![];
    let mut buf = vec![];
    for (i, c) in s.into_iter().enumerate() {
        if c == ')' {
            match buf.last() {
                Some(&('(', j)) => {
                    res.push((j as u32 + 1, i as u32 + 1));
                    buf.pop();
                }
                _ => buf.push((c, i)),
            }
        } else {
            buf.push((c, i));
        }
    }

    res.sort_unstable_by_key(|v| v.1);

    for (l, r) in res {
        println!("{} {}", l, r)
    }
}
