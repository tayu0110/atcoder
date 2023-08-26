use proconio::*;

fn main() {
    input! {mut s: [marker::Chars; 8]}
    s.reverse();

    for i in 0..8 {
        for j in 0..s.len() {
            if s[i][j] == '*' {
                println!("{}{}", (j as u8 + b'a') as char, i + 1);
            }
        }
    }
}
