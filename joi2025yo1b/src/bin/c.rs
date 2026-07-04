use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars, t: marker::Chars}

    let mut a = 0;
    let mut b = 0;
    for (s, t) in s.into_iter().zip(t) {
        if s == t {
            continue;
        }

        if s == 'R' {
            b += 1;
        } else if s == 'S' && t == 'P' {
            a += 1;
        } else {
            b += 1;
        }
    }
    println!("{a} {b}")
}
