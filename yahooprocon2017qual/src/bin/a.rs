use proconio::*;

fn main() {
    input! {mut s: marker::Chars}

    let mut t = "yahoo".chars().collect::<Vec<_>>();
    t.sort();
    s.sort();

    if t == s {
        println!("YES")
    } else {
        println!("NO")
    }
}
