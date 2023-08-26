use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    for i in 0..n {
        if s[i] == '|' {
            for j in i + 1..n {
                if s[j] == '*' {
                    println!("in");
                    return;
                } else if s[j] == '|' {
                    println!("out");
                    return;
                }
            }
        }
    }
}
