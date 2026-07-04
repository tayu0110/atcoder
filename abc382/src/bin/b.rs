use proconio::*;

fn main() {
    input! {n: usize, mut d: usize, mut s: marker::Bytes}

    for i in (0..n).rev() {
        if d > 0 && s[i] == b'@' {
            s[i] = b'.';
            d -= 1;
        }
    }

    println!("{}", s.into_iter().map(|c| c as char).collect::<String>())
}
