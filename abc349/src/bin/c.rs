use proconio::*;

fn main() {
    input! {s: marker::Chars, t: marker::Chars}

    let mut now = 0;
    for c in s {
        let c = c.to_ascii_uppercase();
        if now < t.len() && t[now] == c {
            now += 1;
        }
    }

    if now == 3 || (now == 2 && t[now] == 'X') {
        println!("Yes")
    } else {
        println!("No")
    }
}
