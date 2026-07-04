use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    for i in 1..n {
        if n % i == 0 {
            let t = &s[..i];
            if s.chunks_exact(i).all(|s| s == t) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
