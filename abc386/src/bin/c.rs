use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes, t: marker::Bytes}

    if s.len().abs_diff(t.len()) > 1 {
        println!("No");
        return;
    }

    if s.len() == t.len() {
        if s.iter().zip(t.iter()).filter(|(s, t)| s != t).count() <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if s.len() < t.len() {
        for i in 0..s.len() {
            if s[i] != t[i] {
                if s[i..] == t[i + 1..] {
                    println!("Yes");
                } else {
                    println!("No")
                }
                return;
            }
        }
        println!("Yes")
    } else {
        for i in 0..t.len() {
            if s[i] != t[i] {
                if s[i + 1..] == t[i..] {
                    println!("Yes")
                } else {
                    println!("No")
                }
                return;
            }
        }
        println!("Yes")
    }
}
