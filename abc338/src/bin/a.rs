use proconio::*;

fn main() {
    input! {s: marker::Chars}

    if s[0].is_uppercase() && (s.len() == 1 || s.iter().skip(1).all(|&c| c.is_lowercase())) {
        println!("Yes");
    } else {
        println!("No")
    }
}
