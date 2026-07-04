use proconio::*;

fn main() {
    input! {mut s: marker::Chars}

    for mut i in 1..s.len() {
        while i > 0 && s[i - 1] == 'W' && s[i] == 'A' {
            s[i - 1] = 'A';
            s[i] = 'C';
            i -= 1;
        }
    }

    println!("{}", s.into_iter().collect::<String>())
}
