use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut res = 0;
    let mut pos = s.iter().position(|c| c == &'A').unwrap();
    for c in 'B'..='Z' {
        let new = s.iter().position(|nc| nc == &c).unwrap();
        res += pos.abs_diff(new);
        pos = new;
    }
    println!("{res}")
}
