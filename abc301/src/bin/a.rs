use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    let a = s.iter().filter(|&&c| c == 'A').count();
    let t = n - a;

    if t > a {
        println!("T")
    } else if a > t {
        println!("A")
    } else {
        if s[n - 1] == 'T' {
            println!("A")
        } else {
            println!("T")
        }
    }
}
