use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut res = 0usize;
    let mut prev = false;
    for c in s {
        if !prev && c == 'I' {
            res += 1;
            prev = true;
        } else if prev && c == 'O' {
            res += 1;
            prev = false;
        }
    }

    if prev {
        println!("{res}")
    } else {
        println!("{}", res.saturating_sub(1));
    }
}
