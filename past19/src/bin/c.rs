use proconio::*;

fn main() {
    input! {n: marker::Bytes}

    for i in 0..n.len() {
        for j in b'0'..=b'9' {
            let mut new = n.clone();
            new[i] = j;
            if new.windows(2).all(|v| v[0].abs_diff(v[1]) <= 1) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
