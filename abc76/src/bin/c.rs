use proconio::*;

fn main() {
    input! {s: marker::Chars, t: marker::Chars}
    let n = s.len();
    let m = t.len();

    if n < m {
        println!("UNRESTORABLE");
        return;
    }

    let mut res = vec!['z'; 100].into_iter().collect::<String>();
    for i in 0..=n - m {
        if s[i..i + m]
            .iter()
            .zip(t.iter())
            .filter(|&(&s, _)| s != '?')
            .all(|(s, t)| s == t)
        {
            let mut s = s.clone();
            s[i..i + m].copy_from_slice(&t[..]);
            let s = s
                .into_iter()
                .map(|c| if c == '?' { 'a' } else { c })
                .collect::<String>();
            res = res.min(s);
        }
    }

    if res == vec!['z'; 100].into_iter().collect::<String>() {
        println!("UNRESTORABLE")
    } else {
        println!("{}", res)
    }
}
