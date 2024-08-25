use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut t = s.clone();
    t.sort();
    t.dedup();

    if s.iter().filter(|&&c| c == t[0]).count() == 1 {
        println!("{}", s.iter().position(|c| *c == t[0]).unwrap() + 1)
    } else {
        println!("{}", s.iter().position(|c| *c == t[1]).unwrap() + 1)
    }
}
