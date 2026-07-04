use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let mut ret = 0;
    let mut s = s.into_iter().peekable();
    for t in t {
        while s.next_if_eq(&t).is_none() {
            if t == b'A' {
                ret += 1;
                break;
            } else if s.next_if_eq(&b'A').is_some() {
                ret += 1;
                continue;
            } else {
                println!("-1");
                return;
            }
        }
    }

    while s.next_if_eq(&b'A').is_some() {
        ret += 1;
    }

    if s.next().is_none() {
        println!("{ret}")
    } else {
        println!("-1")
    }
}
