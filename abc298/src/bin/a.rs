use proconio::*;

fn main() {
    input! {_n: usize, s: marker::Chars}

    if s.iter().all(|c| *c == 'o' || *c == '-') && s.iter().any(|c| *c == 'o') {
        println!("Yes")
    } else {
        println!("No")
    }
}
