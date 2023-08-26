use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    let mut d = -1;
    if s.contains(&'D') {
        for i in (0..n).rev() {
            if s[i] == 'D' {
                d = i as i32;
                break;
            }
        }
    }

    let mut k = n as i32;
    if s.contains(&'K') {
        for i in 0..n {
            if s[i] == 'K' {
                k = i as i32;
                break;
            }
        }
    }

    println!("{}", (k - d).max(0));
}
