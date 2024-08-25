use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut b = 0;
    let mut l = 0;
    for &s in &s {
        if s.is_uppercase() {
            b += 1;
        } else {
            l += 1;
        }
    }

    let s = s.into_iter().collect::<String>();
    if b > l {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}
