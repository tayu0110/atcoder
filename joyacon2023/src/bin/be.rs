use proconio::input;
use proconio::marker::Chars;

fn llc(s: Vec<char>) -> Vec<(char, usize)> {
    let mut res = vec![];
    for c in s {
        match res.last_mut() {
            Some((d, cnt)) if *d == c => *cnt += 1,
            _ => res.push((c, 1)),
        }
    }

    res
}

fn main() {
    input! {s: Chars, t: Chars}

    let s = llc(s);
    let t = llc(t);

    if s.len() != t.len() {
        println!("No");
        return;
    }

    for ((sc, scnt), (tc, tcnt)) in s.into_iter().zip(t.into_iter()) {
        if sc != tc {
            println!("No");
            return;
        }

        if tcnt < scnt {
            println!("No");
            return;
        }

        if scnt != tcnt && scnt == 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
